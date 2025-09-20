use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub max_parent_node_width: usize,
    pub max_leaf_node_width: usize,
    pub line_spacing: usize,
    pub initial_depth: usize,
    pub center_lock: bool,
    pub focus_lock: bool,
    pub max_undo_steps: usize,
    pub auto_save: bool,
    pub clipboard_backend: ClipboardBackend,
    pub symbol1: char,
    pub symbol2: char,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ClipboardBackend {
    Os,
    Internal,
    File(String),
}

impl Default for Config {
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
            clipboard_backend: ClipboardBackend::Os,
            symbol1: '✓',
            symbol2: '✗',
        }
    }
}

impl Config {
    pub fn load() -> Self {
        if let Some(config_path) = get_config_path() {
            if let Ok(content) = fs::read_to_string(&config_path) {
                if let Ok(config) = toml::from_str(&content) {
                    return config;
                }
            }
        }
        
        Self::default()
    }
    
    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(config_path) = get_config_path() {
            if let Some(parent) = config_path.parent() {
                fs::create_dir_all(parent)?;
            }
            
            let content = toml::to_string_pretty(self)?;
            fs::write(config_path, content)?;
        }
        
        Ok(())
    }
}

fn get_config_path() -> Option<PathBuf> {
    if let Ok(config_dir) = std::env::var("XDG_CONFIG_HOME") {
        Some(PathBuf::from(config_dir).join("rust-mind-map").join("config.toml"))
    } else if let Some(home_dir) = dirs::home_dir() {
        #[cfg(target_os = "windows")]
        {
            Some(home_dir.join("rust-mind-map").join("config.toml"))
        }
        #[cfg(target_os = "macos")]
        {
            Some(home_dir.join("Library").join("Preferences").join("rust-mind-map").join("config.toml"))
        }
        #[cfg(not(any(target_os = "windows", target_os = "macos")))]
        {
            Some(home_dir.join(".config").join("rust-mind-map").join("config.toml"))
        }
    } else {
        None
    }
}