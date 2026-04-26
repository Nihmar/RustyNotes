// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

/// Application entry point.
/// Delegates to `rustynotes_lib::run()` which sets up the Tauri application.
fn main() {
    rustynotes_lib::run()
}
