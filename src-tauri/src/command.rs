use crate::clip;

// remember to call `.manage(MyState::default())`
#[tauri::command]
#[allow(dead_code)]
pub async fn get_selection_text() -> Result<String, String> {
    clip::read_text()
}
