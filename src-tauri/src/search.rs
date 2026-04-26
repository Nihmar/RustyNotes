use regex::Regex;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use tauri::State;
use walkdir::WalkDir;

use crate::state::ManagedState;

/// A single search match with context snippet and relevance score.
/// Title matches score higher (100) than body matches (50).
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SearchResult {
    pub path: String,
    pub title: String,
    pub snippet: String,
    pub line: usize,
    pub relevance: u32,
}

fn title_from_path(path: &std::path::Path) -> String {
    path.file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("Untitled")
        .to_string()
}

/// Builds a highlighted snippet around a match position with 80 characters of context on each side.
/// The matched text is wrapped in `**` markers for frontend highlighting.
fn snippet_around(text: &str, pos: usize, query_len: usize) -> String {
    let context = 80;
    let start = if pos > context { pos - context } else { 0 };
    let end = std::cmp::min(text.len(), pos + query_len + context);

    let mut snippet = String::from("...");
    if start > 0 {
        snippet.push_str(&text[start..pos]);
    }
    snippet.push_str("**");
    snippet.push_str(&text[pos..pos + query_len]);
    snippet.push_str("**");
    snippet.push_str(&text[pos + query_len..end]);
    snippet.push_str("...");

    snippet
}

fn get_notebook_root(state: &ManagedState) -> Result<PathBuf, String> {
    let app_state = state.lock().map_err(|e| format!("Failed to lock state: {}", e))?;
    match app_state.active_notebook_path.as_ref() {
        Some(root) => Ok(PathBuf::from(root)),
        None => Err("No notebook is open. Please open or create a notebook first.".to_string()),
    }
}

/// Full-text search across all `.md` files in the active notebook.
/// Performs a case-insensitive regex search, returning title matches first (relevance=100)
/// followed by body matches (relevance=50), sorted by relevance then path.
#[tauri::command]
pub fn search_notes(query: String, state: State<'_, ManagedState>) -> Result<Vec<SearchResult>, String> {
    if query.trim().is_empty() {
        return Ok(Vec::new());
    }

    let pattern = format!(r"(?i){}", regex::escape(&query));
    let re = Regex::new(&pattern).map_err(|e| e.to_string())?;

    let mut results: Vec<SearchResult> = Vec::new();
    let root = get_notebook_root(&state)?;

    for entry in WalkDir::new(&root)
        .follow_links(false)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if entry.file_type().is_file() {
            let path = entry.path();
            if path.extension().map_or(false, |e| e == "md") {
                if let Ok(content) = fs::read_to_string(path) {
                    let title = title_from_path(path);
                    let path_str = path
                        .strip_prefix(&root)
                        .unwrap_or(path)
                        .to_string_lossy()
                        .to_string();

                    if re.is_match(&title) {
                        results.push(SearchResult {
                            path: path_str.clone(),
                            title: title.clone(),
                            snippet: format!("Title match: {}", title),
                            line: 0,
                            relevance: 100,
                        });
                    }

                    for (line_num, line_text) in content.lines().enumerate() {
                        if let Some(mat) = re.find(line_text) {
                            let snippet = snippet_around(line_text, mat.start(), mat.len());
                            results.push(SearchResult {
                                path: path_str.clone(),
                                title: title.clone(),
                                snippet,
                                line: line_num + 1,
                                relevance: 50,
                            });
                        }
                    }
                }
            }
        }
    }

    results.sort_by(|a, b| b.relevance.cmp(&a.relevance).then(a.path.cmp(&b.path)));
    Ok(results)
}
