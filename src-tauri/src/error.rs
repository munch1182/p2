#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Error: {0}")]
    Custom(String),
    #[error("IO Error: {0}")]
    Io(#[from] std::io::Error),
    #[error("JSON Error: {0}")]
    Json(#[from] serde_json::Error),
}

pub type Result<T> = std::result::Result<T, Error>;

#[macro_export]
macro_rules! err_str {
    ($fmt:expr, $($arg:tt)*) => {
       Error::Custom(format!($fmt, $($arg)*))
    };
}
