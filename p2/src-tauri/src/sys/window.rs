use tauri::{Manager, WebviewWindow};

use crate::{err, preluad::*, NAME_MAIN_WINDOW};

pub trait WindowExt {
    fn get_window(&self, name: &str) -> Result<WebviewWindow>;
    fn get_main(&self) -> Result<WebviewWindow> {
        self.get_window(NAME_MAIN_WINDOW)
    }
    fn show_window(&self, name: &str, always_on_top: bool) -> Result<()>;
    fn hide_window(&self, name: &str) -> Result<()>;
    fn hide_main(&self) -> Result<()> {
        self.hide_window(NAME_MAIN_WINDOW)
    }
    fn _setup_main_window(&self) -> Result<()> {
        let window = self.get_main()?;
        window.set_focus()?;
        window.set_always_on_top(true)?;
        Ok(())
    }
    fn toggle_show_hide(&self, name: &str) -> Result<()>;
    fn toggle_show_hide_main(&self) -> Result<()> {
        self.toggle_show_hide(NAME_MAIN_WINDOW)
    }
}

impl WindowExt for tauri::AppHandle {
    fn get_window(&self, name: &str) -> Result<WebviewWindow> {
        Ok(self
            .get_webview_window(name)
            .ok_or_else(|| err!("window not found: {name}"))?)
    }

    fn show_window(&self, name: &str, always_on_top: bool) -> Result<()> {
        let window = self.get_window(name)?;
        window.show()?;
        window.set_focus()?;
        window.set_always_on_top(always_on_top)?;
        Ok(())
    }

    fn hide_window(&self, name: &str) -> Result<()> {
        let window = self.get_window(name)?;
        window.hide()?;
        window.set_always_on_top(false)?;
        Ok(())
    }

    fn toggle_show_hide(&self, name: &str) -> Result<()> {
        let window = self.get_window(name)?;
        if window.is_visible().unwrap_or(false) {
            self.hide_window(name)
        } else {
            self.show_window(name, true)
        }
    }
}
