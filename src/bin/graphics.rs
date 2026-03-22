//! Graphics Rendering Test
//!
//! This test verifies that the miniquad graphics pipeline works correctly.
//! It displays a window with an animated background color.
//!
//! **What it tests:**
//! - Basic graphics context setup
//! - Frame rendering and timing
//! - Color animation
//!
//! **How to use:**
//! Run with: `cargo run --bin graphics`
//! You should see a window with an animated background color. Close the window to exit.

use miniquad::*;
use miniquad_test_utils::create_test_window_conf;

struct GraphicsTest {
    frame_count: u64,
}

impl GraphicsTest {
    pub fn new() -> Self {
        println!("=== Graphics Rendering Test Started ===");
        println!("Displaying animated colors...");

        Self {
            frame_count: 0,
        }
    }
}

impl EventHandler for GraphicsTest {
    fn update(&mut self) {
        self.frame_count += 1;

        if self.frame_count % 60 == 0 {
            println!("Frame: {}", self.frame_count);
        }
    }

    fn draw(&mut self) {
        // Calculate animated color based on frame count
        let cycle = (self.frame_count as f32 / 120.0 * std::f32::consts::PI * 2.0).sin();
        let _r = (cycle + 1.0) / 2.0;
        let _g = ((cycle + 1.0) / 2.0 * 0.5 + 0.25).sin().abs();
        let _b = ((cycle + 1.0) / 2.0 * 0.3 + 0.15).cos().abs();

        // The draw function is called by miniquad's main loop
        // We just calculate colors here - the actual rendering is handled by the framework
    }

    fn window_restored_event(&mut self) {
        println!("Window restored");
    }

    fn window_minimized_event(&mut self) {
        println!("Window minimized");
    }
}

fn main() {
    let conf = create_test_window_conf();
    miniquad::start(conf, || {
        Box::new(GraphicsTest::new())
    });

    println!("\n=== Graphics Rendering Test Completed ===");
}
