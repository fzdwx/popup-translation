use std::error::Error;
use std::sync::atomic::AtomicBool;
use once_cell::sync::OnceCell;
use parking_lot::Mutex;

use tauri::{App, AppHandle, GlobalShortcutManager, LogicalPosition, LogicalSize, Manager, PhysicalPosition, PhysicalSize, Runtime};

use crate::{clip, setting, utils};

pub static APP_HANDLE: OnceCell<AppHandle> = OnceCell::new();
pub static ALWAYS_ON_TOP: AtomicBool = AtomicBool::new(false);
pub static CPU_VENDOR: Mutex<String> = Mutex::new(String::new());
pub static SELECTED_TEXT: Mutex<String> = Mutex::new(String::new());
pub static PREVIOUS_PRESS_TIME: Mutex<u128> = Mutex::new(0);
pub static PREVIOUS_RELEASE_TIME: Mutex<u128> = Mutex::new(0);
pub static PREVIOUS_RELEASE_POSITION: Mutex<(i32, i32)> = Mutex::new((0, 0));
pub static RELEASE_THREAD_ID: Mutex<u32> = Mutex::new(0);

pub fn init(app: &mut App) -> Result<(), Box<dyn Error>> {
    APP_HANDLE.get_or_init(|| app.handle());

    let main_window = app.get_window("main").unwrap();
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

    #[cfg(target_os = "linux")]
        let mut manager = mouce::nix::NixMouseManager::new_x11();
    #[cfg(target_os = "windows")]
        let mut manager = Mouse::new();
    #[cfg(target_os = "macos")]
        let mut manager = Mouse::new();

    let hook_result = manager.hook(Box::new(|e| {
        match e {
            mouce::common::MouseEvent::Press(mouce::common::MouseButton::Left) => {
                let current_press_time = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_millis();
                *PREVIOUS_PRESS_TIME.lock() = current_press_time;
            }
            mouce::common::MouseEvent::Release(mouce::common::MouseButton::Left) => {
                let current_release_time = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_millis();
                let mut is_text_selected_event = false;
                let (x, y): (i32, i32) = utils::get_mouse_location().unwrap();
                let (prev_release_x, prev_release_y) = { *PREVIOUS_RELEASE_POSITION.lock() };
                {
                    *PREVIOUS_RELEASE_POSITION.lock() = (x, y);
                }
                let mouse_distance =
                    (((x - prev_release_x).pow(2) + (y - prev_release_y).pow(2)) as f64).sqrt();
                let mut previous_press_time = 0;
                let mut previous_release_time = 0;
                {
                    let previous_press_time_lock = PREVIOUS_PRESS_TIME.lock();
                    let mut previous_release_time_lock = PREVIOUS_RELEASE_TIME.lock();
                    previous_release_time = *previous_release_time_lock;
                    *previous_release_time_lock = current_release_time;
                    previous_press_time = *previous_press_time_lock;
                }
                let is_pressed = previous_release_time < previous_press_time;
                let pressed_time = current_release_time - previous_press_time;
                let is_double_click =
                    current_release_time - previous_release_time < 700 && mouse_distance < 10.0;
                if is_pressed && pressed_time > 300 && mouse_distance > 20.0 {
                    is_text_selected_event = true;
                }
                if previous_release_time != 0 && is_double_click {
                    is_text_selected_event = true;
                }
                let is_click_on_thumb = match APP_HANDLE.get().unwrap().get_window(THUMB_WIN_NAME) {
                    Some(window) => match window.outer_position() {
                        Ok(position) => {
                            let scale_factor = window.scale_factor().unwrap_or(1.0);
                            if let Ok(size) = window.outer_size() {
                                if cfg!(target_os = "macos") {
                                    let LogicalPosition { x: x1, y: y1 } =
                                        position.to_logical::<i32>(scale_factor);
                                    let LogicalSize {
                                        width: mut w,
                                        height: mut h,
                                    } = size.to_logical::<i32>(scale_factor);
                                    if cfg!(target_os = "windows") {
                                        w = (20.0 as f64 * scale_factor) as i32;
                                        h = (20.0 as f64 * scale_factor) as i32;
                                    }
                                    let (x2, y2) = (x1 + w, y1 + h);
                                    let res = x >= x1 && x <= x2 && y >= y1 && y <= y2;
                                    println!("is_click_on_thumb: {}", res);
                                    res
                                } else {
                                    let PhysicalPosition { x: x1, y: y1 } =
                                        position;
                                    let PhysicalSize {
                                        width: mut w,
                                        height: mut h,
                                    } = size;
                                    if cfg!(target_os = "windows") {
                                        w = (20.0 as f64 * scale_factor) as u32;
                                        h = (20.0 as f64 * scale_factor) as u32;
                                    }
                                    let (x2, y2) = (x1 + w as i32, y1 + h as i32);
                                    let res = x >= x1 && x <= x2 && y >= y1 && y <= y2;
                                    res
                                }
                            } else {
                                false
                            }
                        }
                        Err(err) => {
                            println!("err: {:?}", err);
                            false
                        }
                    },
                    None => false,
                };
                // println!("is_text_selected_event: {}", is_text_selected_event);
                // println!("is_click_on_thumb: {}", is_click_on_thumb);
                if !is_text_selected_event && !is_click_on_thumb {
                    close_thumb();
                    // println!("not text selected event");
                    // println!("is_click_on_thumb: {}", is_click_on_thumb);
                    // println!("mouse_distance: {}", mouse_distance);
                    // println!("pressed_time: {}", pressed_time);
                    // println!("released_time: {}", current_release_time - previous_release_time);
                    // println!("is_double_click: {}", is_double_click);
                    return;
                }

                if !is_click_on_thumb {
                    if RELEASE_THREAD_ID.is_locked() {
                        // println!("release thread is locked");
                        return;
                    }
                    let _lock = RELEASE_THREAD_ID.lock();
                    let selected_text = clip::read_text().unwrap_or("".to_string());
                    if !selected_text.is_empty() {
                        {
                            *SELECTED_TEXT.lock() = selected_text;
                        }
                        show_thumb(x, y);
                    } else {
                        // println!("selected text is empty");
                        close_thumb()
                    }
                } else {
                    close_thumb();
                    // let selected_text = (*SELECTED_TEXT.lock()).to_string();
                    // if !selected_text.is_empty() {
                    //     let window = windows::show_main_window(false, false);
                    //     utils::send_text(selected_text);
                    //     if cfg!(target_os = "windows") {
                    //         window.set_always_on_top(true).unwrap();
                    //         let always_on_top = ALWAYS_ON_TOP.load(Ordering::Acquire);
                    //         if !always_on_top {
                    //             std::thread::spawn(move || {
                    //                 window.set_always_on_top(false).unwrap();
                    //             });
                    //         }
                    //     } else {
                    //         window.set_focus().unwrap();
                    //     }
                    // }
                }
            }
            _ => {}
        }
    }));

    #[cfg(target_os = "linux")]
        let hook_result = manager.hook(Box::new(|_e| {}));

    match hook_result {
        Ok(id) => {
            assert_eq!(manager.unhook(id), Ok(()));
        }
        // Hooking may require user privileges on some systems
        // e.g. requires super user for Linux
        Err(err) => assert_eq!(mouce::error::Error::PermissionDenied, err),
    }
    println!("{:?}", manager.get_position());


    let handle = app.handle();
    let mut shortcut_manager = app.global_shortcut_manager();

    let shortcuts = config.shortcuts.unwrap_or_default();
    shortcut_manager.register(&shortcuts.toggle, move || {
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

pub const THUMB_WIN_NAME: &str = "thumb";

pub fn close_thumb() {
    match APP_HANDLE.get().unwrap().get_window(THUMB_WIN_NAME) {
        Some(window) => {
            window.set_always_on_top(false).unwrap();
            window.hide().unwrap();
        }
        None => {}
    }
}

pub fn show_thumb(x: i32, y: i32) {
    let position_offset = 7.0 as f64;
    match APP_HANDLE.get().unwrap().get_window(THUMB_WIN_NAME) {
        Some(window) => {
            println!("Thumb window already exists");
            if cfg!(target_os = "macos") {
                window
                    .set_position(LogicalPosition::new(
                        x as f64 + position_offset,
                        y as f64 + position_offset,
                    ))
                    .unwrap();
            } else {
                window
                    .set_position(PhysicalPosition::new(
                        x as f64 + position_offset,
                        y as f64 + position_offset,
                    ))
                    .unwrap();
            }
            window.unminimize().unwrap();
            window.show().unwrap();
            window.set_always_on_top(true).unwrap();
        }
        None => {
            println!("Thumb window does not exist");
            let builder = tauri::WindowBuilder::new(
                APP_HANDLE.get().unwrap(),
                THUMB_WIN_NAME,
                tauri::WindowUrl::App("thumb.html".into()),
            )
                .fullscreen(false)
                .focused(false)
                .inner_size(20.0, 20.0)
                .min_inner_size(20.0, 20.0)
                .max_inner_size(20.0, 20.0)
                .visible(true)
                .resizable(false)
                .skip_taskbar(true)
                .decorations(false);

            #[cfg(target_os = "macos")]
                let window = builder.hidden_title(true).build().unwrap();

            #[cfg(not(target_os = "macos"))]
                let window = builder.transparent(true).build().unwrap();

            if cfg!(target_os = "macos") {
                window
                    .set_position(LogicalPosition::new(
                        x as f64 + position_offset,
                        y as f64 + position_offset,
                    ))
                    .unwrap();
            } else {
                window
                    .set_position(PhysicalPosition::new(
                        x as f64 + position_offset,
                        y as f64 + position_offset,
                    ))
                    .unwrap();
            }

            #[cfg(target_os = "macos")]
            set_shadow(&window, true).unwrap();

            window.unminimize().unwrap();
            window.show().unwrap();
            window.set_always_on_top(true).unwrap();
        }
    }
}
