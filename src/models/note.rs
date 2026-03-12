use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::time::SystemTime;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Note {
    pub title: String,
    pub content: String,
    pub path: PathBuf,
    pub modified: SystemTime,
}

impl Note {
    pub fn new(title: String, path: PathBuf) -> Self {
        Self {
            title,
            content: String::new(),
            path,
            modified: SystemTime::now(),
        }
    }

    pub fn from_path(path: PathBuf) -> Self {
        let title = path
            .file_stem()
            .map(|s| s.to_string_lossy().to_string())
            .unwrap_or_else(|| "Untitled".to_string());
        Self::new(title, path)
    }

    pub fn set_content(&mut self, content: String) {
        self.content = content;
        self.modified = SystemTime::now();
    }
}
