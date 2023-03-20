use crate::{
    clip,
    setting::{self, Config},
};

#[tauri::command]
#[allow(dead_code)]
pub async fn get_selection_text() -> Result<String, String> {
    clip::read_text()
}

#[tauri::command]
#[allow(dead_code)]
pub async fn write_config(data: String, app_handle: tauri::AppHandle) -> Result<(), String> {
    let (old, new) = Config::cover(data);
    setting::refresh_shortcuts(old, new, app_handle).unwrap();
    Ok(())
}

#[tauri::command]
#[allow(dead_code)]
pub async fn read_config() -> Result<Config, String> {
    Ok(Config::read())
}
