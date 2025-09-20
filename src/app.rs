use crate::node::Node;
use crate::file::FileManager;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Mode {
    Normal,
    Edit,
    Search,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub max_parent_node_width: usize,
    pub max_leaf_node_width: usize,
    pub line_spacing: usize,
    pub initial_depth: usize,
    pub center_lock: bool,
    pub focus_lock: bool,
    pub max_undo_steps: usize,
    pub auto_save: bool,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            max_parent_node_width: 25,
            max_leaf_node_width: 55,
            line_spacing: 1,
            initial_depth: 1,
            center_lock: false,
            focus_lock: false,
            max_undo_steps: 24,
            auto_save: false,
        }
    }
}

#[derive(Debug)]
pub struct App {
    pub root: Node,
    pub active_node_id: Uuid,
    pub mode: Mode,
    pub config: AppConfig,
    pub file_manager: FileManager,
    pub search_query: String,
    pub search_results: Vec<Uuid>,
    pub current_search_index: usize,
    pub edit_buffer: String,
    pub edit_cursor: usize,
    pub clipboard: Vec<Node>,
    pub undo_stack: VecDeque<Node>,
    pub message: Option<String>,
    pub should_quit: bool,
    pub modified: bool,
    pub scroll_offset: usize,
}

impl App {
    pub fn new(filename: Option<String>) -> Result<Self, Box<dyn std::error::Error>> {
        let config = AppConfig::default();
        let file_manager = FileManager::new(filename);
        
        let root = if let Some(ref filename) = file_manager.filename {
            file_manager.load_from_file(filename)?
        } else {
            Node::new_root()
        };

        let active_node_id = root.id;

        Ok(Self {
            root,
            active_node_id,
            mode: Mode::Normal,
            config,
            file_manager,
            search_query: String::new(),
            search_results: Vec::new(),
            current_search_index: 0,
            edit_buffer: String::new(),
            edit_cursor: 0,
            clipboard: Vec::new(),
            undo_stack: VecDeque::new(),
            message: None,
            should_quit: false,
            modified: false,
            scroll_offset: 0,
        })
    }

    pub fn push_undo_state(&mut self) {
        if self.undo_stack.len() >= self.config.max_undo_steps {
            self.undo_stack.pop_front();
        }
        self.undo_stack.push_back(self.root.clone());
    }

    pub fn undo(&mut self) {
        if let Some(previous_state) = self.undo_stack.pop_back() {
            self.root = previous_state;
            // Try to keep the same active node, or default to root
            if self.root.find_node(self.active_node_id).is_none() {
                self.active_node_id = self.root.id;
            }
            self.set_message("Undo successful".to_string());
        } else {
            self.set_message("Nothing to undo".to_string());
        }
    }

    pub fn get_active_node(&self) -> Option<&Node> {
        self.root.find_node(self.active_node_id)
    }

    pub fn get_active_node_mut(&mut self) -> Option<&mut Node> {
        self.root.find_node_mut(self.active_node_id)
    }

    pub fn set_message(&mut self, message: String) {
        self.message = Some(message);
    }

    pub fn clear_message(&mut self) {
        self.message = None;
    }

    pub fn enter_edit_mode(&mut self) {
        let node_text = if let Some(node) = self.get_active_node() {
            node.text.clone()
        } else {
            String::new()
        };
        
        self.mode = Mode::Edit;
        self.edit_buffer = node_text;
        self.edit_cursor = self.edit_buffer.len();
    }

    pub fn exit_edit_mode(&mut self) {
        let buffer_clone = self.edit_buffer.clone();
        self.push_undo_state();
        
        if let Some(node) = self.get_active_node_mut() {
            node.text = buffer_clone;
            self.modified = true;
        }
        
        self.mode = Mode::Normal;
        self.edit_buffer.clear();
        self.edit_cursor = 0;
    }

    pub fn cancel_edit_mode(&mut self) {
        self.mode = Mode::Normal;
        self.edit_buffer.clear();
        self.edit_cursor = 0;
    }

    pub fn enter_search_mode(&mut self) {
        self.mode = Mode::Search;
        self.search_query.clear();
    }

    pub fn exit_search_mode(&mut self) {
        self.mode = Mode::Normal;
    }

    pub fn perform_search(&mut self) {
        if !self.search_query.is_empty() {
            self.search_results = self.root.search(&self.search_query);
            self.current_search_index = 0;
            if !self.search_results.is_empty() {
                self.active_node_id = self.search_results[0];
                self.set_message(format!("Found {} results", self.search_results.len()));
            } else {
                self.set_message("No results found".to_string());
            }
        }
        self.exit_search_mode();
    }

    pub fn next_search_result(&mut self) {
        if !self.search_results.is_empty() {
            self.current_search_index = (self.current_search_index + 1) % self.search_results.len();
            self.active_node_id = self.search_results[self.current_search_index];
            self.set_message(format!(
                "Result {} of {}",
                self.current_search_index + 1,
                self.search_results.len()
            ));
        }
    }

    pub fn previous_search_result(&mut self) {
        if !self.search_results.is_empty() {
            self.current_search_index = if self.current_search_index == 0 {
                self.search_results.len() - 1
            } else {
                self.current_search_index - 1
            };
            self.active_node_id = self.search_results[self.current_search_index];
            self.set_message(format!(
                "Result {} of {}",
                self.current_search_index + 1,
                self.search_results.len()
            ));
        }
    }

    pub fn save(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(ref filename) = self.file_manager.filename {
            self.file_manager.save_to_file(&self.root, filename)?;
            self.modified = false;
            self.set_message(format!("Saved to {}", filename));
        } else {
            self.set_message("No filename specified. Use Save As.".to_string());
        }
        Ok(())
    }

    pub fn save_as(&mut self, filename: String) -> Result<(), Box<dyn std::error::Error>> {
        self.file_manager.filename = Some(filename.clone());
        self.file_manager.save_to_file(&self.root, &filename)?;
        self.modified = false;
        self.set_message(format!("Saved to {}", filename));
        Ok(())
    }

    pub fn add_sibling(&mut self) {
        self.push_undo_state();
        
        // Find parent ID first
        if let Some(parent_id) = self.find_parent_id(self.active_node_id) {
            if let Some(parent) = self.root.find_node_mut(parent_id) {
                let new_id = parent.add_child("New Node".to_string());
                self.active_node_id = new_id;
                self.modified = true;
                self.enter_edit_mode();
            }
        } else if self.active_node_id == self.root.id {
            // If active node is root, add a child instead
            let new_id = self.root.add_child("New Node".to_string());
            self.active_node_id = new_id;
            self.modified = true;
            self.enter_edit_mode();
        }
    }

    pub fn paste_as_sibling(&mut self) {
        if self.clipboard.is_empty() {
            self.set_message("Clipboard is empty".to_string());
            return;
        }

        self.push_undo_state();
        
        if let Some(parent_id) = self.find_parent_id(self.active_node_id) {
            if let Some(parent) = self.root.find_node_mut(parent_id) {
                for node in &self.clipboard {
                    parent.children.push(node.clone());
                }
                self.modified = true;
                self.set_message("Pasted as sibling".to_string());
            }
        }
    }

    fn find_parent_id(&self, child_id: Uuid) -> Option<Uuid> {
        self.find_parent_id_recursive(&self.root, child_id)
    }

    pub fn add_child(&mut self) {
        self.push_undo_state();
        
        if let Some(node) = self.root.find_node_mut(self.active_node_id) {
            let new_id = node.add_child("New Node".to_string());
            self.active_node_id = new_id;
            self.modified = true;
            self.enter_edit_mode();
        }
    }

    pub fn delete_node(&mut self) {
        if self.active_node_id == self.root.id {
            self.set_message("Cannot delete root node".to_string());
            return;
        }

        self.push_undo_state();
        
        // Store deleted node in clipboard
        if let Some(deleted_node) = self.root.remove_child(self.active_node_id) {
            self.clipboard = vec![deleted_node];
            
            // Move to parent or sibling
            let all_nodes = self.root.get_all_nodes();
            if !all_nodes.is_empty() {
                self.active_node_id = all_nodes[0];
            }
            
            self.modified = true;
            self.set_message("Node deleted".to_string());
        }
    }

    pub fn toggle_collapse(&mut self) {
        if let Some(node) = self.root.find_node_mut(self.active_node_id) {
            node.collapsed = !node.collapsed;
        }
    }

    pub fn copy_node(&mut self) {
        if let Some(node) = self.root.find_node(self.active_node_id) {
            self.clipboard = vec![node.clone()];
            self.set_message("Node copied".to_string());
        }
    }

    pub fn paste_as_child(&mut self) {
        if self.clipboard.is_empty() {
            self.set_message("Clipboard is empty".to_string());
            return;
        }

        self.push_undo_state();
        
        if let Some(parent) = self.root.find_node_mut(self.active_node_id) {
            for node in &self.clipboard {
                parent.children.push(node.clone());
            }
            self.modified = true;
            self.set_message("Pasted as child".to_string());
        }
    }

    fn find_parent_id_recursive(&self, node: &Node, child_id: Uuid) -> Option<Uuid> {
        for child in &node.children {
            if child.id == child_id {
                return Some(node.id);
            }
            if let Some(parent_id) = self.find_parent_id_recursive(child, child_id) {
                return Some(parent_id);
            }
        }
        None
    }

    pub fn increase_positive_rank(&mut self) {
        self.push_undo_state();
        if let Some(node) = self.root.find_node_mut(self.active_node_id) {
            node.rank_positive += 1;
            self.modified = true;
        }
    }

    pub fn increase_negative_rank(&mut self) {
        self.push_undo_state();
        if let Some(node) = self.root.find_node_mut(self.active_node_id) {
            node.rank_negative += 1;
            self.modified = true;
        }
    }

    pub fn add_star(&mut self) {
        self.push_undo_state();
        if let Some(node) = self.root.find_node_mut(self.active_node_id) {
            if node.stars < 5 {
                node.stars += 1;
                self.modified = true;
            }
        }
    }

    pub fn remove_star(&mut self) {
        self.push_undo_state();
        if let Some(node) = self.root.find_node_mut(self.active_node_id) {
            if node.stars > 0 {
                node.stars -= 1;
                self.modified = true;
            }
        }
    }

    pub fn toggle_symbol(&mut self) {
        self.push_undo_state();
        if let Some(node) = self.root.find_node_mut(self.active_node_id) {
            node.symbol = match node.symbol {
                None => Some('✓'),
                Some('✓') => Some('✗'),
                Some('✗') => None,
                _ => Some('✓'),
            };
            self.modified = true;
        }
    }

    pub fn collapse_all(&mut self) {
        self.root.collapse_all();
        // Don't collapse the root itself
        self.root.collapsed = false;
    }

    pub fn expand_all(&mut self) {
        self.root.expand_all();
    }

    pub fn collapse_to_level(&mut self, level: usize) {
        self.root.collapse_to_level(level, 0);
    }

    pub fn export_html(&self) -> String {
        fn node_to_html(node: &Node, level: usize) -> String {
            let indent = "  ".repeat(level);
            let mut html = format!("{}<li>{}</li>\n", indent, html_escape::encode_text(&node.text));
            
            if !node.children.is_empty() {
                html = format!("{}<li>\n{}  {}\n", indent, indent, html_escape::encode_text(&node.text));
                html.push_str(&format!("{}  <ul>\n", indent));
                for child in &node.children {
                    html.push_str(&node_to_html(child, level + 2));
                }
                html.push_str(&format!("{}  </ul>\n", indent));
                html.push_str(&format!("{}</li>\n", indent));
            }
            
            html
        }

        let mut html = String::from("<!DOCTYPE html>\n<html>\n<head>\n  <title>Mind Map</title>\n</head>\n<body>\n  <ul>\n");
        html.push_str(&node_to_html(&self.root, 1));
        html.push_str("  </ul>\n</body>\n</html>");
        html
    }
}