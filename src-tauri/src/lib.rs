// Rust module declarations — each corresponds to a backend capability.
mod fs_watcher;
mod notebook;
mod notes;
mod render;
mod search;
mod state;
mod tags;

use std::path::{Path, PathBuf};
use state::ManagedState;
use tauri::Manager;

/// Recursively searches a directory for a file by name.
/// Used by the `vault://` protocol handler to resolve wiki-link image targets.
fn find_file_in_dir(root: &Path, filename: &str) -> Option<PathBuf> {
    for entry in walkdir::WalkDir::new(root)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if entry.file_type().is_file() && entry.file_name() == filename {
            return Some(entry.path().to_path_buf());
        }
    }
    None
}

/// Simple greeting command, used for testing Tauri IPC connectivity.
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

/// Sets up and runs the Tauri application.
///
/// Registers:
/// - Tauri plugins: `opener` (open URLs/files), `dialog` (native file dialogs)
/// - Managed state: `AppState` wrapped in `Mutex`
/// - `vault://` protocol handler for serving embedded images and files
/// - All Tauri commands (notebook CRUD, note CRUD, search, tags)
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .manage(ManagedState::new(state::AppState::new()))
        .register_asynchronous_uri_scheme_protocol("vault", move |ctx, request, responder| {
            let uri = request.uri().to_string();
            let rel_path = if let Some(stripped) = uri.strip_prefix("vault://") {
                stripped.find('/')
                    .map(|i| stripped[i + 1..].to_string())
                    .unwrap_or_default()
            } else {
                String::new()
            };
            if rel_path.is_empty() {
                let _ = responder.respond(
                    tauri::http::Response::builder()
                        .status(400)
                        .header("Content-Type", "text/plain")
                        .body(Vec::new())
                        .unwrap()
                );
                return;
            }
            let app_handle = ctx.app_handle();
            let state = app_handle.state::<ManagedState>();
            let notebook_root = match state.lock().ok().and_then(|s| s.active_notebook_path.clone()) {
                Some(root) => root,
                None => {
                    let _ = responder.respond(
                        tauri::http::Response::builder()
                            .status(500)
                            .header("Content-Type", "text/plain")
                            .body(Vec::new())
                            .unwrap()
                    );
                    return;
                }
            };
            let root = PathBuf::from(&notebook_root);
            let mut file_path = root.join(&rel_path);
            if !file_path.exists() && !rel_path.contains('/') && !rel_path.contains('\\') {
                if let Some(found) = find_file_in_dir(&root, &rel_path) {
                    file_path = found;
                }
            }
            if !file_path.exists() {
                let _ = responder.respond(
                    tauri::http::Response::builder()
                        .status(404)
                        .header("Content-Type", "text/plain")
                        .body(Vec::new())
                        .unwrap()
                );
                return;
            }
            let mime = mime_guess::from_path(&file_path)
                .first_or_octet_stream()
                .to_string();
            match std::fs::read(&file_path) {
                Ok(body) => {
                    let _ = responder.respond(
                        tauri::http::Response::builder()
                            .status(200)
                            .header("Content-Type", mime)
                            .body(body)
                            .unwrap()
                    );
                }
                Err(_) => {
                    let _ = responder.respond(
                        tauri::http::Response::builder()
                            .status(500)
                            .header("Content-Type", "text/plain")
                            .body(Vec::new())
                            .unwrap()
                    );
                }
            }
        })
        .invoke_handler(tauri::generate_handler![
            greet,
            notebook::create_notebook,
            notebook::open_notebook,
            notebook::list_notebooks,
            notebook::close_notebook,
            notebook::get_active_notebook_path,
            notes::list_notes,
            notes::read_note,
            notes::write_note,
            notes::create_note,
            notes::delete_note,
            notes::rename_note,
            notes::move_note,
            search::search_notes,
            tags::get_tags,
            render::render_markdown,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
