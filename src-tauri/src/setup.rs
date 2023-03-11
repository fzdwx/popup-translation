use std::error::Error;

use tauri::{App, GlobalShortcutManager, Manager};

pub fn init(app: &mut App) -> Result<(), Box<dyn Error>> {
    let window = app.get_window("main").unwrap();
    window.hide()?;

    let handle = app.app_handle();
    let mut shortcur = app.global_shortcut_manager();

    shortcur.register("alt+s", move || {
        if window.is_visible().unwrap() {
            window.hide().unwrap();
        } else {
            window.show().unwrap();
        }
    })?;

    Ok(())
}
