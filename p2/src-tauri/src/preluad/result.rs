use std::fmt::format;

use crate::log;

#[derive(Debug, thiserror::Error)]
pub enum Err {
    #[error("Tauri error: {0}")]
    Tauri(#[from] tauri::Error),

    #[error("Tray icon error: {0}")]
    TrayIcon(#[from] tray_icon::Error),

    #[error("Global shortcut error: {0}")]
    GlobalShortcut(#[from] tauri_plugin_global_shortcut::Error),

    #[error("Custom err: {0}")]
    Custom(String),
}

#[macro_export]
macro_rules! err {
    ($($arg:tt)*) => {
        crate::_new_custom_err(format_args!($($arg)*))
    };
}

pub(crate) fn _new_custom_err(args: std::fmt::Arguments) -> Err {
    Err::Custom(format(args))
}

pub type Result<T> = std::result::Result<T, Err>;

pub trait IgnoreErr {
    /**
     * 如果返回Err，则忽略并打印错误信息
     */
    fn ignore_value_err_by_log(self);
}

impl<T> IgnoreErr for Result<T> {
    fn ignore_value_err_by_log(self) {
        match self {
            Ok(_) => {}
            std::result::Result::Err(e) => log!("err: {e}"),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_ignore_value_err_by_log() {
        let res: Result<()> = Err(err!("test"));
        res.ignore_value_err_by_log();
    }
}
