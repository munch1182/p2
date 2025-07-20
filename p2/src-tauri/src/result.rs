#[derive(Debug, thiserror::Error)]
pub enum Err {
    #[error("Tauri error")]
    Tauri(#[from] tauri::Error),

    #[error("Tray icon error")]
    TrayIcon(#[from] tray_icon::Error),

    #[error("Global shortcut error")]
    GlobalShortcut(#[from] tauri_plugin_global_shortcut::Error),

    #[error("Custom error")]
    Custom(String),
}

pub type Result<T> = std::result::Result<T, Err>;
