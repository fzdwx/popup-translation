use crate::{clip, setting::Config};

#[tauri::command]
#[allow(dead_code)]
pub async fn get_selection_text() -> Result<String, String> {
    clip::read_text()
}

#[tauri::command]
#[allow(dead_code)]
pub async fn write_config(data: String) -> Result<(), String> {
    println!("data:{:?}", data);
    Config::cover(data);
    Ok(())
}

#[tauri::command]
#[allow(dead_code)]
pub async fn read_config() -> Result<Config, String> {
    Ok(Config::read())
}
