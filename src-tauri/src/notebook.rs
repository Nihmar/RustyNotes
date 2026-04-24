use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use tauri::{AppHandle, State};

use crate::state::ManagedState;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Notebook {
    pub name: String,
    pub path: String,
    pub created: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NotebookInfo {
    pub name: String,
    pub path: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct NotebookConfig {
    name: String,
    created: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
struct GlobalSettings {
    #[serde(default)]
    recent_notebooks: Vec<NotebookInfo>,
    #[serde(default)]
    theme: Option<String>,
    #[serde(default)]
    default_editor_mode: Option<String>,
    #[serde(default)]
    sidebar_visible: Option<bool>,
    #[serde(default)]
    font_size: Option<u32>,
    #[serde(default)]
    autosave_interval_ms: Option<u64>,
}

fn config_dir() -> PathBuf {
    #[cfg(target_os = "windows")]
    {
        let appdata = std::env::var("APPDATA").unwrap_or_else(|_| ".".to_string());
        PathBuf::from(appdata).join("rustynotes")
    }
    #[cfg(not(target_os = "windows"))]
    {
        let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
        PathBuf::from(home).join(".config").join("rustynotes")
    }
}

fn global_settings_path() -> PathBuf {
    config_dir().join("settings.json")
}

fn load_global_settings() -> GlobalSettings {
    let path = global_settings_path();
    if path.exists() {
        let content = fs::read_to_string(&path).unwrap_or_default();
        serde_json::from_str(&content).unwrap_or_default()
    } else {
        GlobalSettings::default()
    }
}

fn save_global_settings(settings: &GlobalSettings) {
    let path = global_settings_path();
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).ok();
    }
    if let Ok(json) = serde_json::to_string_pretty(settings) {
        fs::write(&path, json).ok();
    }
}

#[tauri::command]
pub fn create_notebook(name: String, path: String, state: State<'_, ManagedState>, app_handle: AppHandle) -> Result<Notebook, String> {
    let nb_path = PathBuf::from(&path);
    if nb_path.exists() && !nb_path.as_os_str().is_empty() {
        let rustynotes_dir = nb_path.join(".rustynotes");
        fs::create_dir_all(&rustynotes_dir).map_err(|e| e.to_string())?;
    }

    let created = chrono::Utc::now().to_rfc3339();

    let config = NotebookConfig {
        name: name.clone(),
        created: created.clone(),
    };

    let config_json = serde_json::to_string_pretty(&config).map_err(|e| e.to_string())?;
    let config_path = nb_path.join(".rustynotes").join("config.json");
    fs::write(&config_path, config_json).map_err(|e| e.to_string())?;

    let notebook = Notebook {
        name: name.clone(),
        path: path.clone(),
        created,
    };

    // Update recent notebooks in global settings
    let mut settings = load_global_settings();
    settings.recent_notebooks.retain(|n| n.path != path);
    settings.recent_notebooks.insert(
        0,
        NotebookInfo {
            name,
            path,
        },
    );
    if settings.recent_notebooks.len() > 10 {
        settings.recent_notebooks.truncate(10);
    }
    save_global_settings(&settings);

    // Set as active
    if let Ok(mut app_state) = state.lock() {
        app_state.active_notebook_path = Some(notebook.path.clone());
        let _ = app_state.watcher.start(&notebook.path, app_handle);
    }

    Ok(notebook)
}

#[tauri::command]
pub fn open_notebook(path: String, state: State<'_, ManagedState>, app_handle: AppHandle) -> Result<Notebook, String> {
    let nb_path = PathBuf::from(&path);
    let config_path = nb_path.join(".rustynotes").join("config.json");

    if !config_path.exists() {
        return Err("Notebook config not found. Please create a notebook first.".to_string());
    }

    let config_content = fs::read_to_string(&config_path).map_err(|e| e.to_string())?;
    let config: NotebookConfig = serde_json::from_str(&config_content).map_err(|e| e.to_string())?;

    let notebook = Notebook {
        name: config.name,
        path: path.clone(),
        created: config.created,
    };

    // Update recent notebooks in global settings
    let mut settings = load_global_settings();
    settings.recent_notebooks.retain(|n| n.path != path);
    settings.recent_notebooks.insert(
        0,
        NotebookInfo {
            name: notebook.name.clone(),
            path: path.clone(),
        },
    );
    if settings.recent_notebooks.len() > 10 {
        settings.recent_notebooks.truncate(10);
    }
    save_global_settings(&settings);

    if let Ok(mut app_state) = state.lock() {
        app_state.active_notebook_path = Some(path.clone());
        let _ = app_state.watcher.start(&path, app_handle);
    }

    Ok(notebook)
}

#[tauri::command]
pub fn list_notebooks() -> Result<Vec<NotebookInfo>, String> {
    let settings = load_global_settings();
    Ok(settings.recent_notebooks)
}

#[tauri::command]
pub fn get_active_notebook_path(state: State<'_, ManagedState>) -> Result<Option<String>, String> {
    let app_state = state.lock().map_err(|e| format!("Failed to lock state: {}", e))?;
    Ok(app_state.active_notebook_path.clone())
}

#[tauri::command]
pub fn close_notebook(state: State<'_, ManagedState>) -> Result<(), String> {
    if let Ok(mut app_state) = state.lock() {
        app_state.active_notebook_path = None;
        app_state.watcher.stop();
    }
    Ok(())
}
