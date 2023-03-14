// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod clip;
mod command;
mod setting;
mod setup;
mod utils;

fn main() {
    tauri::Builder::default()
        .plugin(setting::log_builder().build())
        .setup(setup::init)
        .invoke_handler(tauri::generate_handler![
            command::get_selection_text,
            command::write_config,
            command::read_config
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
