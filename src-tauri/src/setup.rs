use std::error::Error;

use tauri::{App, GlobalShortcutManager, Manager};

use crate::setting;

pub fn init(app: &mut App) -> Result<(), Box<dyn Error>> {
    let main_window = app.get_window("main").unwrap();
    main_window.hide()?;
    main_window.set_decorations(false)?;

    let config = setting::Config::read();

    // // 仅在 macOS 下执行
    // #[cfg(target_os = "macos")]
    // window_vibrancy::apply_vibrancy(&main_window, NSVisualEffectMaterial::FullScreenUI)
    //     .expect("Unsupported platform! 'apply_vibrancy' is only supported on macOS");

    // // 仅在 windows 下执行
    // #[cfg(target_os = "windows")]
    // window_vibrancy::apply_blur(&main_window, Some((18, 18, 18, 125)))
    //     .expect("Unsupported platform! 'apply_blur' is only supported on Windows");

    // WindowBuilder::new(
    //     &app.handle(),
    //     "core",
    //     tauri::WindowUrl::App("http://www.bing.com/dict/search".into()),
    // )
    // .build();

    let handle = app.handle();
    let mut shortcur = app.global_shortcut_manager();

    let shortcuts = config.shortcuts.unwrap_or_default();
    shortcur.register(&shortcuts.toggle, move || {
        if main_window.is_visible().unwrap() {
            main_window.hide().unwrap();
        } else {
            main_window.show().unwrap();
            main_window.set_focus().unwrap();
            handle.emit_all("refresh-translation", "ttt").unwrap();
        }
    })?;

    Ok(())
}
