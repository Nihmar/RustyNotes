use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use tauri::State;
use walkdir::WalkDir;

use crate::state::ManagedState;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TagInfo {
    pub name: String,
    pub count: u32,
}

fn get_notebook_root(state: &ManagedState) -> Result<PathBuf, String> {
    let app_state = state.lock().map_err(|e| format!("Failed to lock state: {}", e))?;
    match app_state.active_notebook_path.as_ref() {
        Some(root) => Ok(PathBuf::from(root)),
        None => Err("No notebook is open. Please open or create a notebook first.".to_string()),
    }
}

#[tauri::command]
pub fn get_tags(state: State<'_, ManagedState>) -> Result<Vec<TagInfo>, String> {
    let tag_re = Regex::new(r"#([\w/-]+)").map_err(|e| e.to_string())?;
    let code_block_re = Regex::new(r"```[\s\S]*?```").unwrap();
    let inline_code_re = Regex::new(r"`[^`]+`").unwrap();

    let mut tag_counts: HashMap<String, u32> = HashMap::new();
    let root = get_notebook_root(&state)?;

    for entry in WalkDir::new(&root)
        .follow_links(false)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if entry.file_type().is_file() {
            let path = entry.path();
            if path.extension().map_or(false, |e| e == "md") {
                if let Ok(mut content) = fs::read_to_string(path) {
                    content = code_block_re.replace_all(&content, "").to_string();
                    content = inline_code_re.replace_all(&content, "").to_string();

                    let mut seen: HashMap<String, bool> = HashMap::new();

                    for cap in tag_re.captures_iter(&content) {
                        if let Some(m) = cap.get(1) {
                            let tag = m.as_str().to_string();
                            if !seen.contains_key(&tag) {
                                seen.insert(tag.clone(), true);
                                *tag_counts.entry(tag).or_insert(0) += 1;
                            }
                        }
                    }
                }
            }
        }
    }

    let mut tags: Vec<TagInfo> = tag_counts
        .into_iter()
        .map(|(name, count)| TagInfo { name, count })
        .collect();
    tags.sort_by(|a, b| b.count.cmp(&a.count).then(a.name.cmp(&b.name)));
    Ok(tags)
}
