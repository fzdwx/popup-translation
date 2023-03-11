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
    use enigo::{Enigo, Key, KeyboardControllable};
    // simulate keyboard operation `Command + c`
    let mut enigo = Enigo::new();

    std::thread::sleep(std::time::Duration::from_millis(200));
    enigo.key_down(Key::Command);
    enigo.key_click(Key::Layout('c'));
    enigo.key_up(Key::Command);

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
    use clipboard::{ClipboardContext, ClipboardProvider};
    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    
    ctx.get_contents().map_err(|e| e.to_string())
}
