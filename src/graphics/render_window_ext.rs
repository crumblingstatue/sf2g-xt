use sf2g::{
    graphics::{RenderTarget, RenderWindow},
    system::Vector2,
    window::{ContextSettings, Style, VideoMode},
};

/// Extensions for `RenderWindow`
pub trait RenderWindowExt {
    /// Center the window on screen
    fn center(&mut self);
    /// Enter a fake fullscreen mode (desktop fullscreen)
    fn desktop_fullscreen(&mut self, title: &str, ctx_settings: &ContextSettings);
}

impl RenderWindowExt for RenderWindow {
    fn center(&mut self) {
        let Vector2 { x, y } = self.size();
        let desktop_mode = VideoMode::desktop_mode();
        let width_diff = desktop_mode.width - x;
        let height_diff = desktop_mode.height - y;
        let new_x = width_diff / 2;
        let new_y = height_diff / 2;
        self.set_position(Vector2::new(new_x as i32, new_y as i32));
    }
    fn desktop_fullscreen(&mut self, title: &str, ctx_settings: &ContextSettings) {
        let desktop_mode = VideoMode::desktop_mode();
        self.recreate(desktop_mode, title, Style::NONE, ctx_settings);
        self.set_position((0, 0).into());
    }
}
