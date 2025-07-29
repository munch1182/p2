use tauri::{
    menu::{MenuBuilder, MenuItem},
    tray::TrayIconBuilder,
};

use crate::{preluad::*, sys::window::WindowExt as _};

pub fn setup_trayicon(app: &tauri::App) -> Result<()> {
    let show = MenuItem::with_id(app, "menu_show", "显示", true, None::<&str>)?;
    let quit = MenuItem::with_id(app, "menu_quit", "退出", true, None::<&str>)?;
    let menu = MenuBuilder::with_id(app, "tray_menu")
        .item(&show)
        .separator()
        .item(&quit)
        .build()?;

    let _ = TrayIconBuilder::new()
        .icon(
            app.default_window_icon()
                .ok_or_else(|| Err::Custom("Failed to get default window icon".into()))?
                .clone(),
        )
        .menu(&menu)
        .on_menu_event(|app, event| match event.id.as_ref() {
            "menu_quit" => {
                log!("quit");
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
