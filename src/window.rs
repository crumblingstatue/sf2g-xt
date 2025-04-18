use {sf2g::window::mouse, std::collections::HashSet};

use sf2g::window::{Event, Key};

/// Input abstraction that collects input state from events.
///
/// Unlike Key::is_pressed(), events only trigger if the window is focused
#[derive(Default)]
pub struct InputState {
    keys_down: HashSet<Key>,
    mouse_down: HashSet<mouse::Button>,
    mouse_pressed: HashSet<mouse::Button>,
}

impl InputState {
    /// Call at start of frame to clear "pressed" state
    pub fn start_frame(&mut self) {
        self.mouse_pressed.clear();
    }
    /// Update from an `Event`. Use this in your event polling loop.
    pub fn update_from_event(&mut self, event: &Event) {
        match event {
            Event::KeyPressed { code, .. } => {
                self.keys_down.insert(*code);
            }
            Event::KeyReleased { code, .. } => {
                self.keys_down.remove(code);
            }
            Event::MouseButtonPressed { button, .. } => {
                self.mouse_down.insert(*button);
                self.mouse_pressed.insert(*button);
            }
            Event::MouseButtonReleased { button, .. } => {
                self.mouse_down.remove(button);
            }
            _ => {}
        }
    }
    /// Returns whether `key` is being held down
    pub fn key_down(&self, key: Key) -> bool {
        self.keys_down.contains(&key)
    }
    /// Returns whether `button` is being held down
    pub fn mouse_down(&self, button: mouse::Button) -> bool {
        self.mouse_down.contains(&button)
    }
    /// Returns whether `button` was pressed this frame
    pub fn mouse_pressed(&self, button: mouse::Button) -> bool {
        self.mouse_pressed.contains(&button)
    }
}
