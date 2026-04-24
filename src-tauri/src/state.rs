use std::sync::Mutex;
use crate::fs_watcher::FsWatcher;

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

pub type ManagedState = Mutex<AppState>;
