mod fs_watcher;
mod notebook;
mod notes;
mod search;
mod state;
mod tags;

use state::ManagedState;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .manage(ManagedState::new(state::AppState::new()))
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
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
