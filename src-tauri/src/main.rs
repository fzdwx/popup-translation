// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod clip;
mod command;
mod setup;

use std::path::PathBuf;

use tauri_plugin_log::{
    fern::colors::{Color, ColoredLevelConfig},
    LogTarget,
};

pub fn app_root() -> PathBuf {
    tauri::api::path::home_dir()
        .unwrap()
        .join(".popup-translation")
}

fn log() -> tauri_plugin_log::Builder {
    tauri_plugin_log::Builder::default()
        .targets([
            // LogTarget::LogDir,
            // LOG PATH: ~/.popup-translation
            LogTarget::Folder(app_root()),
            LogTarget::Stdout,
            LogTarget::Webview,
        ])
        .level(log::LevelFilter::Debug)
        .with_colors(ColoredLevelConfig {
            error: Color::Red,
            warn: Color::Yellow,
            debug: Color::Blue,
            info: Color::BrightGreen,
            trace: Color::Cyan,
        })
}

fn main() {
    tauri::Builder::default()
        .plugin(log().build())
        .setup(setup::init)
        .invoke_handler(tauri::generate_handler![command::get_selection_text])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
