use crate::node::Node;
use std::fs;
use std::io::{BufRead, BufReader, Write};

#[derive(Debug)]
pub struct FileManager {
    pub filename: Option<String>,
}

impl FileManager {
    pub fn new(filename: Option<String>) -> Self {
        Self { filename }
    }

    pub fn load_from_file(&self, filename: &str) -> Result<Node, Box<dyn std::error::Error>> {
        let file = fs::File::open(filename)?;
        let reader = BufReader::new(file);
        
        let mut root = Node::new_root();
        let mut node_stack: Vec<(usize, Node)> = Vec::new();

        for line in reader.lines() {
            let line = line?;
            if line.trim().is_empty() {
                continue;
            }

            let level = count_leading_tabs(&line);
            let text = line.trim().to_string();
            
            // Parse any metadata from the text
            let (clean_text, metadata) = parse_metadata(&text);
            
            let mut new_node = Node::new(clean_text);
            apply_metadata(&mut new_node, metadata);

            // Remove nodes from stack that are at equal or deeper level
            while let Some((stack_level, _)) = node_stack.last() {
                if *stack_level >= level {
                    node_stack.pop();
                } else {
                    break;
                }
            }

            // Add the new node to the appropriate parent
            if level == 0 {
                // Top-level node - replace root or make it root's child
                if root.text == "root" && root.children.is_empty() {
                    root = new_node.clone();
                } else {
                    root.children.push(new_node.clone());
                }
                node_stack.push((level, new_node));
            } else {
                // Find the parent and add as child
                if let Some((_, ref mut parent)) = node_stack.last_mut() {
                    parent.children.push(new_node.clone());
                }
                node_stack.push((level, new_node));
            }
        }

        // Rebuild the tree from the stack
        if let Some((_, final_root)) = node_stack.into_iter().find(|(level, _)| *level == 0) {
            Ok(final_root)
        } else if !root.children.is_empty() {
            Ok(root)
        } else {
            // If we only have root with no content, use filename as root text
            let stem = std::path::Path::new(filename)
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("Untitled");
            root.text = stem.to_string();
            Ok(root)
        }
    }

    pub fn save_to_file(&self, root: &Node, filename: &str) -> Result<(), Box<dyn std::error::Error>> {
        let content = self.serialize_tree(root, 0);
        let mut file = fs::File::create(filename)?;
        file.write_all(content.as_bytes())?;
        Ok(())
    }

    fn serialize_tree(&self, node: &Node, level: usize) -> String {
        let mut result = String::new();
        
        // Add current node (skip root if it's the only node at level 0)
        if level > 0 || !node.children.is_empty() {
            let indent = "\t".repeat(level);
            let metadata = format_metadata(node);
            let line = if metadata.is_empty() {
                format!("{}{}\n", indent, node.text)
            } else {
                format!("{}{} {}\n", indent, node.text, metadata)
            };
            result.push_str(&line);
        }

        // Add children
        for child in &node.children {
            result.push_str(&self.serialize_tree(child, if level == 0 && node.children.len() == 1 { 0 } else { level + 1 }));
        }

        result
    }
}

fn count_leading_tabs(line: &str) -> usize {
    line.chars().take_while(|&c| c == '\t').count()
}

fn parse_metadata(text: &str) -> (String, NodeMetadata) {
    let mut metadata = NodeMetadata::default();
    
    // Look for metadata patterns like [rank: +2/-1, stars: 3]
    if let Some(bracket_start) = text.find('[') {
        if let Some(bracket_end) = text.find(']') {
            let clean_text = format!("{} {}", 
                text[..bracket_start].trim(), 
                text[bracket_end + 1..].trim()
            ).trim().to_string();
            
            let metadata_str = &text[bracket_start + 1..bracket_end];
            
            if let Some(rank_match) = regex::Regex::new(r"rank:\s*([+\-]?\d+)/([+\-]?\d+)")
                .unwrap()
                .captures(metadata_str) 
            {
                if let (Ok(pos), Ok(neg)) = (
                    rank_match[1].parse::<i32>(),
                    rank_match[2].parse::<i32>()
                ) {
                    metadata.rank_positive = pos;
                    metadata.rank_negative = neg;
                }
            }
            
            // Parse stars
            if let Some(stars_match) = regex::Regex::new(r"stars:\s*(\d+)")
                .unwrap()
                .captures(metadata_str) 
            {
                if let Ok(stars) = stars_match[1].parse::<u8>() {
                    metadata.stars = stars.min(5);
                }
            }
            
            return (clean_text, metadata);
        }
    }
    
    // Check for symbol prefixes
    let text = text.trim();
    if text.starts_with('✓') {
        metadata.symbol = Some('✓');
        return (text[1..].trim().to_string(), metadata);
    } else if text.starts_with('✗') {
        metadata.symbol = Some('✗');
        return (text[1..].trim().to_string(), metadata);
    }
    
    (text.to_string(), metadata)
}

fn apply_metadata(node: &mut Node, metadata: NodeMetadata) {
    node.rank_positive = metadata.rank_positive;
    node.rank_negative = metadata.rank_negative;
    node.stars = metadata.stars;
    node.symbol = metadata.symbol;
}

fn format_metadata(node: &Node) -> String {
    let mut parts = Vec::new();
    
    if node.rank_positive != 0 || node.rank_negative != 0 {
        parts.push(format!("rank: {}/{}", 
            if node.rank_positive >= 0 { format!("+{}", node.rank_positive) } else { node.rank_positive.to_string() },
            if node.rank_negative >= 0 { format!("+{}", node.rank_negative) } else { node.rank_negative.to_string() }
        ));
    }
    
    if node.stars > 0 {
        parts.push(format!("stars: {}", node.stars));
    }
    
    if !parts.is_empty() {
        format!("[{}]", parts.join(", "))
    } else {
        String::new()
    }
}

#[derive(Default)]
struct NodeMetadata {
    rank_positive: i32,
    rank_negative: i32,
    stars: u8,
    symbol: Option<char>,
}