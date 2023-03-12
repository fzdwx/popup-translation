use std::error::Error;

use tauri::{App, GlobalShortcutManager, Manager};

pub fn init(app: &mut App) -> Result<(), Box<dyn Error>> {
    let main_window = app.get_window("main").unwrap();
    main_window.hide()?;

    // WindowBuilder::new(
    //     &app.handle(),
    //     "core",
    //     tauri::WindowUrl::App("http://www.bing.com/dict/search".into()),
    // )
    // .build();

    let handle = app.handle();
    let mut shortcur = app.global_shortcut_manager();

    shortcur.register("alt+s", move || {
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
