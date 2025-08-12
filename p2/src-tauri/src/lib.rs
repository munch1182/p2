#![allow(unused)]

mod command;
mod plugin;
mod preluad;
mod search;
mod sys;
mod utils;

use crate::command::*;
use crate::preluad::*;
use crate::sys::{setup_shortcut, setup_trayicon};

const NAME_MAIN_WINDOW: &str = "main";

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| setup_main_window(app))
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![log, save_window_loc, reset_window])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn setup_main_window(
    app: &tauri::App,
) -> std::result::Result<(), Box<(dyn std::error::Error + 'static)>> {
    setup_trayicon(app).ignore_value_err_by_log();
    setup_shortcut(app).ignore_value_err_by_log();
    Ok(())
}
