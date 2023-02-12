pub fn read_text() -> Result<String, String> {
    let mut clipboard = arboard::Clipboard::new().unwrap();
    clipboard.get_text().map_err(|err| err.to_string())
}

// fn write_text(text: String) -> Result<(), String> {
//     let mut clipboard = arboard::Clipboard::new().unwrap();
//     clipboard.set_text(text).map_err(|err| err.to_string())
// }