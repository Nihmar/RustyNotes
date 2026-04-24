use regex::Regex;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NoteMeta {
    pub title: String,
    pub path: String,
    pub modified: String,
    pub size: u64,
    pub tags: Vec<String>,
}

fn title_from_path(path: &Path) -> String {
    path.file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("Untitled")
        .to_string()
}

fn extract_tags(content: &str) -> Vec<String> {
    let re = Regex::new(r"#([\w/-]+)").unwrap();
    let mut tags: Vec<String> = re
        .captures_iter(content)
        .filter_map(|cap| cap.get(1).map(|m| m.as_str().to_string()))
        .collect();
    tags.sort();
    tags.dedup();
    tags
}

fn meta_from_path(path: &Path) -> Option<NoteMeta> {
    let metadata = path.metadata().ok()?;
    let modified = metadata
        .modified()
        .ok()
        .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
        .map(|d| {
            chrono::DateTime::from_timestamp(d.as_secs() as i64, d.subsec_nanos())
                .map(|dt| dt.to_rfc3339())
                .unwrap_or_default()
        })
        .unwrap_or_default();

    let title = title_from_path(&path);
    let path_str = path.to_string_lossy().to_string();
    let tags = if let Ok(content) = fs::read_to_string(&path) {
        extract_tags(&content)
    } else {
        Vec::new()
    };

    Some(NoteMeta {
        title,
        path: path_str,
        modified,
        size: metadata.len(),
        tags,
    })
}

#[tauri::command]
pub fn list_notes(dir: Option<String>) -> Result<Vec<NoteMeta>, String> {
    let base = dir.unwrap_or_else(|| ".".to_string());
    let base_path = PathBuf::from(&base);

    if !base_path.exists() {
        return Err(format!("Directory not found: {}", base));
    }

    let mut notes: Vec<NoteMeta> = Vec::new();

    for entry in WalkDir::new(&base_path)
        .follow_links(false)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if entry.file_type().is_file() {
            let path = entry.path();
            if let Some(ext) = path.extension() {
                if ext == "md" {
                    if let Some(meta) = meta_from_path(path) {
                        notes.push(meta);
                    }
                }
            }
        }
    }

    notes.sort_by(|a, b| a.title.to_lowercase().cmp(&b.title.to_lowercase()));
    Ok(notes)
}

#[tauri::command]
pub fn read_note(path: String) -> Result<String, String> {
    fs::read_to_string(&path).map_err(|e| format!("Failed to read note: {}", e))
}

#[tauri::command]
pub fn write_note(path: String, content: String) -> Result<(), String> {
    let p = PathBuf::from(&path);
    if let Some(parent) = p.parent() {
        fs::create_dir_all(parent).map_err(|e| format!("Failed to create directory: {}", e))?;
    }
    let tmp = format!("{}.tmp", path);
    fs::write(&tmp, &content).map_err(|e| format!("Failed to write note: {}", e))?;
    fs::rename(&tmp, &path).map_err(|e| format!("Failed to save note: {}", e))?;
    Ok(())
}

#[tauri::command]
pub fn create_note(dir: String, title: String) -> Result<NoteMeta, String> {
    let dir_path = PathBuf::from(&dir);
    fs::create_dir_all(&dir_path).map_err(|e| format!("Failed to create directory: {}", e))?;

    let filename = format!("{}.md", title);
    let file_path = dir_path.join(&filename);

    if file_path.exists() {
        return Err(format!("Note already exists: {}", file_path.display()));
    }

    let content = format!("# {}\n\n", title);
    fs::write(&file_path, &content).map_err(|e| format!("Failed to create note: {}", e))?;

    meta_from_path(&file_path).ok_or_else(|| "Failed to read created note metadata".to_string())
}

#[tauri::command]
pub fn delete_note(path: String) -> Result<(), String> {
    let p = PathBuf::from(&path);
    if !p.exists() {
        return Err(format!("Note not found: {}", path));
    }
    fs::remove_file(&p).map_err(|e| format!("Failed to delete note: {}", e))
}

#[tauri::command]
pub fn rename_note(path: String, new_name: String) -> Result<(), String> {
    let p = PathBuf::from(&path);
    if !p.exists() {
        return Err(format!("Note not found: {}", path));
    }

    let parent = p
        .parent()
        .unwrap_or_else(|| Path::new("."))
        .to_path_buf();
    let new_filename = format!("{}.md", new_name);
    let new_path = parent.join(&new_filename);

    if new_path.exists() {
        return Err(format!("A note named '{}' already exists", new_name));
    }

    fs::rename(&p, &new_path).map_err(|e| format!("Failed to rename note: {}", e))
}

#[tauri::command]
pub fn move_note(from: String, to: String) -> Result<(), String> {
    let from_path = PathBuf::from(&from);
    let to_path = PathBuf::from(&to);

    if !from_path.exists() {
        return Err(format!("Source note not found: {}", from));
    }

    if let Some(parent) = to_path.parent() {
        fs::create_dir_all(parent).map_err(|e| format!("Failed to create target directory: {}", e))?;
    }

    fs::rename(&from_path, &to_path).map_err(|e| format!("Failed to move note: {}", e))
}
