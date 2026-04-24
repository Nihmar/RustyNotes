use regex::Regex;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use tauri::State;
use walkdir::WalkDir;

use crate::state::ManagedState;

fn get_notebook_root(state: &ManagedState) -> Result<PathBuf, String> {
    let app_state = state.lock().map_err(|e| format!("Failed to lock state: {}", e))?;
    match app_state.active_notebook_path.as_ref() {
        Some(root) => Ok(PathBuf::from(root)),
        None => Err("No notebook is open. Please open or create a notebook first.".to_string()),
    }
}

fn resolve_path(state: &ManagedState, path: &str) -> Result<PathBuf, String> {
    let p = Path::new(path);
    if p.is_absolute() {
        return Ok(p.to_path_buf());
    }
    let root = get_notebook_root(state)?;
    Ok(root.join(path))
}

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

fn meta_from_path(path: &Path, root: &Path) -> Option<NoteMeta> {
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
    let path_str = path
        .strip_prefix(root)
        .unwrap_or(path)
        .to_string_lossy()
        .to_string();
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
pub fn list_notes(dir: Option<String>, state: State<'_, ManagedState>) -> Result<Vec<NoteMeta>, String> {
    let root = get_notebook_root(&state)?;
    let subdir = dir.unwrap_or_else(|| ".".to_string());
    let base_path = root.join(&subdir);

    if !base_path.exists() {
        return Err(format!("Directory not found: {}", base_path.display()));
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
                    if let Some(meta) = meta_from_path(path, &root) {
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
pub fn read_note(path: String, state: State<'_, ManagedState>) -> Result<String, String> {
    let resolved = resolve_path(&state, &path)?;
    fs::read_to_string(&resolved)
        .map_err(|e| format!("Failed to read note {}: {}", resolved.display(), e))
}

#[tauri::command]
pub fn write_note(path: String, content: String, state: State<'_, ManagedState>) -> Result<(), String> {
    let resolved = resolve_path(&state, &path)?;
    if let Some(parent) = resolved.parent() {
        fs::create_dir_all(parent).map_err(|e| format!("Failed to create directory: {}", e))?;
    }
    let tmp = resolved.with_extension("md.tmp");
    fs::write(&tmp, &content).map_err(|e| format!("Failed to write note: {}", e))?;
    fs::rename(&tmp, &resolved).map_err(|e| format!("Failed to save note: {}", e))?;
    Ok(())
}

#[tauri::command]
pub fn create_note(dir: String, title: String, state: State<'_, ManagedState>) -> Result<NoteMeta, String> {
    let root = get_notebook_root(&state)?;
    let dir_path = resolve_path(&state, &dir)?;
    fs::create_dir_all(&dir_path).map_err(|e| format!("Failed to create directory: {}", e))?;

    let filename = format!("{}.md", title);
    let file_path = dir_path.join(&filename);

    if file_path.exists() {
        return Err(format!("Note already exists: {}", file_path.display()));
    }

    let content = format!("# {}\n\n", title);
    fs::write(&file_path, &content).map_err(|e| format!("Failed to create note: {}", e))?;

    meta_from_path(&file_path, &root).ok_or_else(|| "Failed to read created note metadata".to_string())
}

#[tauri::command]
pub fn delete_note(path: String, state: State<'_, ManagedState>) -> Result<(), String> {
    let resolved = resolve_path(&state, &path)?;
    if !resolved.exists() {
        return Err(format!("Note not found: {}", resolved.display()));
    }
    fs::remove_file(&resolved).map_err(|e| format!("Failed to delete note: {}", e))
}

#[tauri::command]
pub fn rename_note(path: String, new_name: String, state: State<'_, ManagedState>) -> Result<(), String> {
    let resolved = resolve_path(&state, &path)?;
    if !resolved.exists() {
        return Err(format!("Note not found: {}", resolved.display()));
    }

    let parent = resolved
        .parent()
        .unwrap_or_else(|| Path::new("."))
        .to_path_buf();
    let new_filename = format!("{}.md", new_name);
    let new_path = parent.join(&new_filename);

    if new_path.exists() {
        return Err(format!("A note named '{}' already exists", new_name));
    }

    fs::rename(&resolved, &new_path).map_err(|e| format!("Failed to rename note: {}", e))
}

#[tauri::command]
pub fn move_note(from: String, to: String, state: State<'_, ManagedState>) -> Result<(), String> {
    let from_path = resolve_path(&state, &from)?;
    let to_path = resolve_path(&state, &to)?;

    if !from_path.exists() {
        return Err(format!("Source note not found: {}", from_path.display()));
    }

    if let Some(parent) = to_path.parent() {
        fs::create_dir_all(parent).map_err(|e| format!("Failed to create target directory: {}", e))?;
    }

    fs::rename(&from_path, &to_path).map_err(|e| format!("Failed to move note: {}", e))
}
