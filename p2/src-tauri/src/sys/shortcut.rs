use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt as _, Modifiers, Shortcut};

use crate::{preluad::*, sys::window::WindowExt as _};

pub fn setup_shortcut(app: &tauri::App) -> Result<()> {
    log!("setup shortcut");
    let ctrlg = Shortcut::new(Some(Modifiers::CONTROL), Code::KeyG);
    let esc = Shortcut::new(Some(Modifiers::empty()), Code::Escape);
    let _ = app.handle().plugin(
        tauri_plugin_global_shortcut::Builder::new()
            .with_handler(move |_app, shortcut, event| {
                if event.state() == global_hotkey::HotKeyState::Released {
                    log!("shortcut {:?} triggered {:?}", shortcut, event);

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
