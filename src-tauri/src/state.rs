use std::sync::Mutex;
use crate::fs_watcher::FsWatcher;

/// Global application state held in Tauri's managed state system.
///
/// Tracks the currently active notebook path and the file system watcher instance.
/// Wrapped in `ManagedState` (a `Mutex<AppState>`) for thread-safe interior mutability.
pub struct AppState {
    pub active_notebook_path: Option<String>,
    pub watcher: FsWatcher,
}

impl AppState {
    pub fn new() -> Self {
        AppState {
            active_notebook_path: None,
            watcher: FsWatcher::new(),
        }
    }
}

/// Thread-safe handle to AppState, shared across all Tauri commands.
pub type ManagedState = Mutex<AppState>;
