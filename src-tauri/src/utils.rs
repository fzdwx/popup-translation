pub fn exists(path: &std::path::Path) -> bool {
    std::path::Path::new(path).exists()
}

pub fn create_file(path: &std::path::Path) -> anyhow::Result<std::fs::File> {
    if let Some(p) = path.parent() {
        std::fs::create_dir_all(p)?
    }
    std::fs::File::create(path).map_err(Into::into)
}

// ~/.popup-translation
pub fn app_root() -> std::path::PathBuf {
    tauri::api::path::home_dir()
        .unwrap()
        .join(".popup-translation")
}
