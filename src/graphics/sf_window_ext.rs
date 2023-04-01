use sfml::{
    graphics::{RenderTarget, RenderWindow},
    system::Vector2,
    window::VideoMode,
};

pub trait SfWindowExt {
    /// Center the window on screen
    fn center(&mut self);
}

impl SfWindowExt for RenderWindow {
    fn center(&mut self) {
        let Vector2 { x, y } = self.size();
        let desktop_mode = VideoMode::desktop_mode();
        let width_diff = desktop_mode.width - x;
        let height_diff = desktop_mode.height - y;
        let new_x = width_diff / 2;
        let new_y = height_diff / 2;
        self.set_position(Vector2::new(new_x as i32, new_y as i32));
    }
}
