// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod clip;
mod command;
mod setting;
mod setup;
mod utils;
mod sys_tray;

fn main() {
    tauri::Builder::default()
        .plugin(setting::log_builder().build())
        .setup(setup::init)
        .invoke_handler(tauri::generate_handler![
            command::get_selection_text,
            command::write_config,
            command::read_config
        ])
        .system_tray(sys_tray::Tray::init())
        .on_system_tray_event(|app, event|{
            sys_tray::Tray::system_tray_event(app, event);
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
