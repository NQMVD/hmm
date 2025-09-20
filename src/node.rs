use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Node {
    pub id: Uuid,
    pub text: String,
    pub children: Vec<Node>,
    pub collapsed: bool,
    pub rank_positive: i32,
    pub rank_negative: i32,
    pub stars: u8,
    pub symbol: Option<char>,
    pub hidden: bool,
}

impl Node {
    pub fn new(text: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            text,
            children: Vec::new(),
            collapsed: false,
            rank_positive: 0,
            rank_negative: 0,
            stars: 0,
            symbol: None,
            hidden: false,
        }
    }

    pub fn new_root() -> Self {
        Self::new("root".to_string())
    }

    pub fn add_child(&mut self, text: String) -> Uuid {
        let child = Node::new(text);
        let id = child.id;
        self.children.push(child);
        id
    }

    pub fn find_node_mut(&mut self, id: Uuid) -> Option<&mut Node> {
        if self.id == id {
            return Some(self);
        }
        for child in &mut self.children {
            if let Some(node) = child.find_node_mut(id) {
                return Some(node);
            }
        }
        None
    }

    pub fn find_node(&self, id: Uuid) -> Option<&Node> {
        if self.id == id {
            return Some(self);
        }
        for child in &self.children {
            if let Some(node) = child.find_node(id) {
                return Some(node);
            }
        }
        None
    }

    pub fn find_parent_mut(&mut self, child_id: Uuid) -> Option<&mut Node> {
        // Use a different approach - find parent by UUID matching
        self.find_parent_by_child_id_mut(child_id)
    }

    fn find_parent_by_child_id_mut(&mut self, child_id: Uuid) -> Option<&mut Node> {
        // Check if any direct child has the target ID
        for child in &self.children {
            if child.id == child_id {
                // This is a hack to work around borrowing - we'll use unsafe
                // In a real implementation, we'd restructure to avoid this
                return unsafe { Some(&mut *(self as *mut Node)) };
            }
        }
        
        // Recursively check children
        for child in &mut self.children {
            if let Some(parent) = child.find_parent_by_child_id_mut(child_id) {
                return Some(parent);
            }
        }
        
        None
    }

    pub fn remove_child(&mut self, id: Uuid) -> Option<Node> {
        if let Some(pos) = self.children.iter().position(|child| child.id == id) {
            Some(self.children.remove(pos))
        } else {
            for child in &mut self.children {
                if let Some(removed) = child.remove_child(id) {
                    return Some(removed);
                }
            }
            None
        }
    }

    pub fn get_all_nodes(&self) -> Vec<Uuid> {
        let mut nodes = vec![self.id];
        for child in &self.children {
            nodes.extend(child.get_all_nodes());
        }
        nodes
    }

    pub fn search(&self, query: &str) -> Vec<Uuid> {
        let mut results = Vec::new();
        if self.text.to_lowercase().contains(&query.to_lowercase()) {
            results.push(self.id);
        }
        for child in &self.children {
            results.extend(child.search(query));
        }
        results
    }

    pub fn get_level(&self, target_id: Uuid, current_level: usize) -> Option<usize> {
        if self.id == target_id {
            return Some(current_level);
        }
        for child in &self.children {
            if let Some(level) = child.get_level(target_id, current_level + 1) {
                return Some(level);
            }
        }
        None
    }

    pub fn collapse_all(&mut self) {
        self.collapsed = true;
        for child in &mut self.children {
            child.collapse_all();
        }
    }

    pub fn expand_all(&mut self) {
        self.collapsed = false;
        for child in &mut self.children {
            child.expand_all();
        }
    }

    pub fn collapse_to_level(&mut self, target_level: usize, current_level: usize) {
        if current_level >= target_level {
            self.collapsed = true;
        } else {
            self.collapsed = false;
        }
        for child in &mut self.children {
            child.collapse_to_level(target_level, current_level + 1);
        }
    }
}