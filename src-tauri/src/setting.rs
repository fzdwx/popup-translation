use std::path::PathBuf;

use tauri::{GlobalShortcutManager, Manager};
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

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct Config {
    pub keys: KeyInfo,
    pub mode: Option<Mode>,
    pub shortcuts: Option<Shortcuts>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            keys: KeyInfo::default(),
            mode: Some(Mode::default()),
            shortcuts: Some(Shortcuts::default()),
        }
    }
}

#[derive(Default, serde::Deserialize, serde::Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct KeyInfo {
    pub chat_gpt: String,
    pub youdao: String,
    pub google: String,
}

#[derive(Default, serde::Deserialize, serde::Serialize, Debug)]
pub enum Mode {
    #[serde(rename = "aggergate")]
    #[default]
    Aggergate,
    #[serde(rename = "split")]
    Split,
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct Shortcuts {
    pub toggle: String, // default: alt+s
}

impl Default for Shortcuts {
    fn default() -> Self {
        Self {
            toggle: "alt+s".to_string(),
        }
    }
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

    pub fn cover(data: String) -> (Self, Self) {
        log::debug!("config.json cover data: {}", data);

        let old = Self::read();
        log::debug!(
            "config.json cover old config: {}",
            serde_json::to_string_pretty(&old).unwrap()
        );

        let s = serde_json::from_str(&data)
            .unwrap_or_else(|err| {
                log::error!("[cover] config parse error: {}", err);
                Self::default()
            })
            .write();

        log::debug!(
            "config.json cover new config: {}",
            serde_json::to_string_pretty(&s).unwrap()
        );

        (old, s)
    }
}

pub fn refresh_shortcuts(_old: Config, new: Config, app: tauri::AppHandle) -> anyhow::Result<()> {
    let mut manager = app.global_shortcut_manager();
    let main_window = app.get_window("main").unwrap();

    manager.unregister_all()?;
    let shortcuts = new.shortcuts.unwrap_or_default();
    manager.register(&shortcuts.toggle, move || {
        if main_window.is_visible().unwrap() {
            main_window.hide().unwrap();
        } else {
            main_window.show().unwrap();
            main_window.set_focus().unwrap();
            app.emit_all("refresh-translation", "ttt").unwrap();
        }
    })?;

    Ok(())
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
