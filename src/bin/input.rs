//! Input Handling Test
//!
//! This test verifies that keyboard and mouse input events are properly detected
//! and handled by miniquad.
//!
//! **What it tests:**
//! - Keyboard press/release events
//! - Mouse movement events
//! - Mouse button press/release events
//! - Character input events
//! - Input event logging and display
//!
//! **How to use:**
//! Run with: `cargo run --bin input`
//! Press keys and move your mouse/click to see events logged to the console.
//! Close the window to exit.

use miniquad::*;
use miniquad_test_utils::create_test_window_conf;
use std::collections::VecDeque;

const MAX_LOG_ENTRIES: usize = 20;

struct InputTest {
    event_log: VecDeque<String>,
    keys_pressed: std::collections::HashSet<KeyCode>,
    last_mouse_pos: (f32, f32),
}

impl InputTest {
    pub fn new() -> Self {
        println!("=== Input Handling Test Started ===");
        println!("Actions:");
        println!("  - Press any keyboard key");
        println!("  - Move your mouse");
        println!("  - Click mouse buttons");
        println!("  - Type characters");
        println!("  - Close window to exit");
        println!();

        Self {
            event_log: VecDeque::new(),
            keys_pressed: std::collections::HashSet::new(),
            last_mouse_pos: (0.0, 0.0),
        }
    }

    fn log_event(&mut self, event: String) {
        println!("{}", event);
        self.event_log.push_back(event);
        if self.event_log.len() > MAX_LOG_ENTRIES {
            self.event_log.pop_front();
        }
    }
}

impl EventHandler for InputTest {
    fn update(&mut self) {
        // Update loop - nothing special for input test
    }

    fn draw(&mut self) {
        // Drawing happens in miniquad's render loop
    }

    fn key_down_event(&mut self, keycode: KeyCode, _modifiers: KeyMods, _repeat: bool) {
        if self.keys_pressed.insert(keycode) {
            self.log_event(format!("KEY DOWN: {:?}", keycode));
        }
    }

    fn key_up_event(&mut self, keycode: KeyCode, _modifiers: KeyMods) {
        if self.keys_pressed.remove(&keycode) {
            self.log_event(format!("KEY UP: {:?}", keycode));
        }
    }

    fn char_event(&mut self, character: char, _modifiers: KeyMods, _repeat: bool) {
        self.log_event(format!("CHAR: '{}'", character));
    }

    fn mouse_motion_event(&mut self, x: f32, y: f32) {
        self.last_mouse_pos = (x, y);
        // Log every 20 pixels of movement to reduce spam
        if ((x - self.last_mouse_pos.0).abs() + (y - self.last_mouse_pos.1).abs()) > 20.0 {
            self.log_event(format!("MOUSE MOVE: ({:.0}, {:.0})", x, y));
        }
    }

    fn mouse_button_down_event(&mut self, button: MouseButton, x: f32, y: f32) {
        self.log_event(format!("MOUSE DOWN: {:?} at ({:.0}, {:.0})", button, x, y));
    }

    fn mouse_button_up_event(&mut self, button: MouseButton, x: f32, y: f32) {
        self.log_event(format!("MOUSE UP: {:?} at ({:.0}, {:.0})", button, x, y));
    }

    fn mouse_wheel_event(&mut self, x: f32, y: f32) {
        self.log_event(format!("SCROLL: x={:.2}, y={:.2}", x, y));
    }

    fn window_minimized_event(&mut self) {
        self.log_event("WINDOW: Minimized".to_string());
    }

    fn window_restored_event(&mut self) {
        self.log_event("WINDOW: Restored".to_string());
    }
}

fn main() {
    let conf = create_test_window_conf();
    miniquad::start(conf, || Box::new(InputTest::new()));

    println!("\n=== Input Handling Test Completed ===");
}
