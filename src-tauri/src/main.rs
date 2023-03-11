// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod clip;
mod command;
mod setup;

fn main() {
    tauri::Builder::default()
        .setup(setup::init)
        .invoke_handler(tauri::generate_handler![command::get_selection_text])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
