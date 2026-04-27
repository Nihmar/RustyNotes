use std::path::{Path, PathBuf};

use regex::Regex;

#[derive(Debug, Clone)]
pub struct NoteEntry {
    pub path: PathBuf,
    pub title: String,
    pub tags: Vec<String>,
    pub wikilinks: Vec<String>,
    pub has_frontmatter: bool,
}

#[derive(Debug, Clone)]
pub struct TreeNode {
    pub name: String,
    pub path: PathBuf,
    pub is_dir: bool,
    pub children: Vec<TreeNode>,
}

pub fn scan_vault(vault_path: &Path) -> Vec<NoteEntry> {
    let mut notes = Vec::new();
    let wikilink_re = Regex::new(r"\[\[([^\]]+)\]\]").unwrap();
    let tag_re = Regex::new(r"#([a-zA-Z0-9_\-/]+)").unwrap();

    for entry in walkdir::WalkDir::new(vault_path)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if entry.file_type().is_file() {
            if let Some(ext) = entry.path().extension() {
                if ext == "md" {
                    let title = entry
                        .path()
                        .file_stem()
                        .unwrap_or_default()
                        .to_string_lossy()
                        .to_string();

                    let content = std::fs::read_to_string(entry.path()).unwrap_or_default();
                    let (tags, wikilinks, has_frontmatter) =
                        parse_note_metadata(&content, &wikilink_re, &tag_re);

                    notes.push(NoteEntry {
                        path: entry.path().to_owned(),
                        title,
                        tags,
                        wikilinks,
                        has_frontmatter,
                    });
                }
            }
        }
    }

    notes
}

fn parse_note_metadata(content: &str, wikilink_re: &Regex, tag_re: &Regex) -> (Vec<String>, Vec<String>, bool) {
    let mut tags = Vec::new();
    let mut wikilinks = Vec::new();
    let mut has_frontmatter = false;

    let lines: Vec<&str> = content.lines().collect();
    if lines.len() >= 2 && lines[0].trim() == "---" {
        has_frontmatter = true;
        if let Some(end) = lines[1..].iter().position(|l| l.trim() == "---") {
            let frontmatter = &lines[1..=end];
            for line in frontmatter {
                let trimmed = line.trim();
                if let Some(value) = trimmed.strip_prefix("tags:") {
                    let parsed: Vec<String> = value
                        .split(',')
                        .map(|t| t.trim().trim_matches('"').trim_matches('\'').to_string())
                        .filter(|t| !t.is_empty())
                        .collect();
                    tags.extend(parsed);
                }
            }
        }
    }

    for cap in wikilink_re.captures_iter(content) {
        wikilinks.push(cap[1].to_string());
    }

    for cap in tag_re.captures_iter(content) {
        tags.push(cap[1].to_string());
    }

    (tags, wikilinks, has_frontmatter)
}

pub fn build_file_tree(vault_path: &Path) -> Vec<TreeNode> {
    build_tree_recursive(vault_path, 0, Some(5))
}

fn build_tree_recursive(path: &Path, depth: usize, max_depth: Option<usize>) -> Vec<TreeNode> {
    if let Some(max) = max_depth {
        if depth >= max {
            return Vec::new();
        }
    }

    let mut children = Vec::new();

    if let Ok(entries) = std::fs::read_dir(path) {
        let mut dirs = Vec::new();
        let mut files = Vec::new();

        for entry in entries.flatten() {
            let name = entry.file_name().to_string_lossy().to_string();
            let entry_path = entry.path();
            if let Ok(ft) = entry.file_type() {
                if ft.is_dir() {
                    // Skip hidden directories
                    if !name.starts_with('.') {
                        dirs.push((name, entry_path));
                    }
                } else if ft.is_file()
                    && entry_path.extension().map_or(false, |e| e == "md")
                {
                    files.push((name, entry_path));
                }
            }
        }

        // Sort alphabetically, dirs first
        dirs.sort_by(|a, b| a.0.to_lowercase().cmp(&b.0.to_lowercase()));
        files.sort_by(|a, b| a.0.to_lowercase().cmp(&b.0.to_lowercase()));

        for (name, dir_path) in dirs {
            children.push(TreeNode {
                name,
                path: dir_path.clone(),
                is_dir: true,
                children: build_tree_recursive(&dir_path, depth + 1, max_depth),
            });
        }

        for (name, file_path) in files {
            children.push(TreeNode {
                name,
                path: file_path,
                is_dir: false,
                children: Vec::new(),
            });
        }
    }

    children
}
