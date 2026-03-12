use crate::models::note::Note;
use std::fs;
use std::path::{Path, PathBuf};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum FolderError {
    #[error("Failed to read directory: {0}")]
    ReadError(#[from] std::io::Error),
    #[error("Invalid path: {0}")]
    InvalidPath(String),
}

pub struct FolderManager {
    pub root: PathBuf,
}

impl FolderManager {
    pub fn new(root: PathBuf) -> Result<Self, FolderError> {
        if !root.exists() {
            return Err(FolderError::InvalidPath(root.to_string_lossy().to_string()));
        }
        if !root.is_dir() {
            return Err(FolderError::InvalidPath(format!(
                "{} is not a directory",
                root.display()
            )));
        }
        Ok(Self { root })
    }

    pub fn scan_notes(&self) -> Result<Vec<Note>, FolderError> {
        let mut notes = Vec::new();
        self.scan_directory(&self.root, &mut notes)?;
        Ok(notes)
    }

    fn scan_directory(&self, dir: &Path, notes: &mut Vec<Note>) -> Result<(), FolderError> {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                self.scan_directory(&path, notes)?;
            } else if let Some(ext) = path.extension() {
                if ext == "md" || ext == "markdown" {
                    notes.push(Note::from_path(path));
                }
            }
        }
        Ok(())
    }
}
