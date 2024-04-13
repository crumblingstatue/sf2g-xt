use {sfml::window::mouse, std::collections::HashSet};

use sfml::window::{Event, Key};

/// Input abstraction that collects input state from events.
///
/// Unlike Key::is_pressed(), events only trigger if the window is focused
#[derive(Default)]
pub struct InputState {
    keys_down: HashSet<Key>,
    mouse_down: HashSet<mouse::Button>,
}

impl InputState {
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
}
