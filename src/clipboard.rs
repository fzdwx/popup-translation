use wry::application::clipboard::Clipboard;

#[cfg(target_os = "linux")]
pub fn read_text() -> Result<String, String> {
    use std::time::Duration;
    use x11_clipboard::Clipboard;

    let clipboard = Clipboard::new().unwrap();
    if let Ok(curr) = clipboard.load(
        clipboard.getter.atoms.primary,
        clipboard.getter.atoms.utf8_string,
        clipboard.getter.atoms.property,
        Duration::from_millis(100),
    ) {
        let curr = String::from_utf8_lossy(&curr)
            .trim_matches('\u{0}')
            .trim()
            .to_string();
        if !curr.is_empty() {
            Ok(curr)
        } else {
            read_text_cross()
        }
    } else {
        read_text_cross()
    }
}

#[cfg(target_os = "macos")]
pub fn read_text() -> Result<String, String> {
    read_text_cross()
}

#[cfg(target_os = "windows")]
pub fn read_text() -> Result<String, String> {
    use enigo::{Enigo, Key, KeyboardControllable};
    // simulate keyboard operation `Ctrl+c`
    let mut enigo = Enigo::new();

    std::thread::sleep(std::time::Duration::from_millis(200));
    enigo.key_down(Key::Control);
    enigo.key_click(Key::Layout('c'));
    enigo.key_up(Key::Control);

    read_text_cross()
}

fn read_text_cross() -> Result<String, String> {
    wry::application::clipboard::Clipboard::new()
        .read_text()
        .ok_or("Unsupported clipboard".to_string())
}

// fn write_text(text: String) -> Result<(), String> {
//     let mut clipboard = arboard::Clipboard::new().unwrap();
//     clipboard.set_text(text).map_err(|err| err.to_string())
// }
