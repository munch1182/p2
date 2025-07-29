use crate::{preluad::*, sys::window::WindowExt};
use serde::Serialize;
use tauri::{AppHandle, LogicalSize};

#[tauri::command]
pub fn log(str: &str) {
    println!("{}", str);
}

#[tauri::command]
pub fn save_window_loc(handle: AppHandle) -> InvokeResult<()> {
    _save_window_loc(handle).into()
}

#[tauri::command]
pub fn reset_window(handle: AppHandle) -> InvokeResult<()> {
    _reset_window(handle).into()
}

fn _reset_window(handle: AppHandle) -> Result<()> {
    log!("reset window");
    let app = handle.get_main()?;
    let size = LogicalSize::new(600, 57);
    app.set_size(size)?;
    app.center()?;
    Ok(())
}

fn _save_window_loc(handle: AppHandle) -> Result<()> {
    log!("save window loc");
    let app = handle.get_main()?;
    let size = app.inner_size()?;
    let loc = app.inner_position()?;
    log!("size: {:?}, loc: {:?}", size, loc);
    Ok(())
}

#[derive(Serialize)]
pub struct InvokeResult<T: Serialize> {
    code: u8,
    result: Option<T>,
}

impl<T: Serialize> InvokeResult<T> {
    pub fn new(any: T) -> Self {
        Self {
            code: 0,
            result: Some(any),
        }
    }

    pub fn err(code: u8) -> Self {
        Self { code, result: None }
    }
}

impl<T> From<Result<T>> for InvokeResult<T>
where
    T: Serialize,
{
    fn from(value: Result<T>) -> Self {
        match value {
            Ok(v) => InvokeResult::new(v),
            Err(_) => InvokeResult::err(1),
        }
    }
}
