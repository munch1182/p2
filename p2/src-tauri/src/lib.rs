use tauri::menu::{Menu, MenuItem};
use tauri::tray::TrayIconBuilder;
use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Modifiers, Shortcut};

mod log;
mod preluad;
mod result;
mod window;

use crate::preluad::*;
use crate::window::*;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            if let Err(err) = setup_main_window(app) {
                log!("Failed to setup tray icon: {}", err);
            }
            if let Err(err) = setup_trayicon(app) {
                log!("Failed to setup tray icon: {}", err);
            }
            if let Err(err) = setup_shortcut(app) {
                log!("Failed to setup shortcut: {}", err);
            }
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn setup_main_window(app: &tauri::App) -> Result<()> {
    app.handle().setup_main_window()?;
    Ok(())
}

fn setup_trayicon(app: &tauri::App) -> Result<()> {
    let show = MenuItem::with_id(app, "menu_show", "显示", true, None::<&str>)?;
    let quit = MenuItem::with_id(app, "menu_quit", "退出", true, None::<&str>)?;
    let menu = Menu::with_items(app, &[&show, &quit])?;

    let _ = TrayIconBuilder::new()
        .icon(
            app.default_window_icon()
                .ok_or_else(|| Err::Custom("Failed to get default window icon".into()))?
                .clone(),
        )
        .menu(&menu)
        .on_menu_event(|app, event| match event.id.as_ref() {
            "menu_quit" => {
                app.exit(0);
            }
            "menu_show" => {
                if let Ok(window) = app.get_main() {
                    let _ = window.show();
                }
            }
            _ => {}
        })
        .build(app)?;
    Ok(())
}

fn setup_shortcut(app: &tauri::App) -> Result<()> {
    println!("setup shortcut");
    let ctrlg = Shortcut::new(Some(Modifiers::CONTROL), Code::KeyG);
    let esc = Shortcut::new(Some(Modifiers::empty()), Code::Escape);
    let _ = app.handle().plugin(
        tauri_plugin_global_shortcut::Builder::new()
            .with_handler(move |_app, shortcut, event| {
                log!("shortcut {:?} triggered {:?}", shortcut, event);

                if event.state() == global_hotkey::HotKeyState::Released {
                    if shortcut == &ctrlg {
                        let _ = _app.toggle_show_hide_main();
                    } else if shortcut == &esc {
                        let _ = _app.hide_main();
                    }
                }
            })
            .build(),
    );
    app.global_shortcut().register(ctrlg)?;
    app.global_shortcut().register(esc)?;
    return Ok(());
}
