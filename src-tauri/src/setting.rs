use std::path::PathBuf;

use tauri_plugin_log::{
    fern::colors::{Color, ColoredLevelConfig},
    LogTarget,
};

use crate::utils;

fn config_path() -> PathBuf {
    utils::app_root().join("config.json")
}

fn log_path() -> PathBuf {
    utils::app_root().join("log")
}

#[derive(Default, serde::Deserialize, serde::Serialize, Debug)]
pub struct Config {
    pub keys: KeyInfo,
}

#[derive(Default, serde::Deserialize, serde::Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct KeyInfo {
    pub chat_gpt: String,
    pub youdao: String,
    pub google: String,
}

impl Config {
    pub fn read() -> Self {
        if !utils::exists(&config_path()) {
            log::info!("config.json not found");
            Self::default().write();
        }

        match std::fs::read_to_string(config_path()) {
            Ok(v) => serde_json::from_str(&v).unwrap_or_else(|err| {
                log::error!("[read] config parse error: {}", err);
                Self::default()
            }),
            Err(err) => {
                log::error!("config.json read error: {}", err);
                Self::default()
            }
        }
    }

    pub fn write(self) -> Self {
        let path = &config_path();
        if !utils::exists(path) {
            utils::create_file(path).unwrap();
            log::info!("confg.json created");
        }
        if let Ok(v) = serde_json::to_string_pretty(&self) {
            std::fs::write(path, v).unwrap_or_else(|err| {
                log::error!("config.json write error: {}", err);
                Self::default().write();
            });
        } else {
            log::error!("config.json serialize error");
        }

        self
    }

    pub fn cover(data: String) -> Self {
        log::debug!("config.json cover new config: {}", data);
        log::debug!(
            "config.json cover old config: {}",
            serde_json::to_string_pretty(&Self::read()).unwrap()
        );
        serde_json::from_str(&data)
            .unwrap_or_else(|err| {
                log::error!("[cover] config parse error: {}", err);
                Self::default()
            })
            .write()
    }
}

pub fn log_builder() -> tauri_plugin_log::Builder {
    tauri_plugin_log::Builder::default()
        .targets([
            LogTarget::Folder(log_path()),
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
