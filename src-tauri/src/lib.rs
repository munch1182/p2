use serde_json::Value;

mod error;
mod prelude;
use crate::prelude::*;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
async fn receive_file(file: Value) {
    println!("---- {}", 1111);
    let file = receive_file_data_2_file(file);
}

fn receive_file_data_2_file(file: Value) -> Result<()> {
    let name = file.get("name");
    let data = file.get("data").is_none();
    println!("---- {:?}, {}", name, data);
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    println!("Hello, World!");
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![receive_file])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

pub trait Option2Result<T> {
    fn to_result<F>(self, err: F) -> Result<T>
    where
        F: FnOnce() -> Error;
}

impl<T> Option2Result<T> for Option<T> {
    fn to_result<F>(self, err: F) -> Result<T>
    where
        F: FnOnce() -> Error,
    {
        match self {
            Some(v) => Ok(v),
            None => Err(err()),
        }
    }
}
