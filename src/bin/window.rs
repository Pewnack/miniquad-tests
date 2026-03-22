//! Window Creation and Lifecycle Test
//!
//! This test verifies that a basic miniquad window can be created, displayed,
//! and properly responds to lifecycle events (focus, resize, close).
//!
//! **What it tests:**
//! - Window creation with default configuration
//! - Window display and rendering loop
//! - Proper handling of application lifecycle
//! - Frame timing
//!
//! **How to use:**
//! Run with: `cargo run --bin window`
//! Close the window with the X button to exit the test.

use miniquad::*;
use miniquad_test_utils::{create_test_window_conf, FrameTimer};

struct WindowTest {
    frame_timer: FrameTimer,
    frame_count: u64,
    clear_color: [f32; 4],
}

impl WindowTest {
    pub fn new() -> Self {
        println!("=== Window Creation Test Started ===");
        println!("Creating window...");
        Self {
            frame_timer: FrameTimer::new(),
            frame_count: 0,
            clear_color: [0.1, 0.2, 0.3, 1.0],
        }
    }
}

impl EventHandler for WindowTest {
    fn update(&mut self) {
        self.frame_timer.tick();
        self.frame_count += 1;

        // Print stats every 60 frames (~1 second at 60 FPS)
        if self.frame_count % 60 == 0 {
            println!(
                "Frame {}: {:.2} FPS, avg frame time: {:.2}ms",
                self.frame_count,
                self.frame_timer.average_fps(),
                self.frame_timer.average_frame_time().as_secs_f64() * 1000.0
            );
        }
    }

    fn draw(&mut self) {
        // Animate the background color slightly for visual feedback
        self.clear_color[0] = (self.frame_count as f32 * 0.001).sin() * 0.2 + 0.3;
    }

    fn mouse_motion_event(&mut self, x: f32, y: f32) {
        // Update clear color based on mouse position (normalized to 0-1)
        self.clear_color[0] = (x / 800.0).min(1.0);
        self.clear_color[1] = (y / 600.0).min(1.0);
    }

    fn window_minimized_event(&mut self) {
        println!("Window minimized");
    }

    fn window_restored_event(&mut self) {
        println!("Window restored");
    }

    fn resize_event(&mut self, width: f32, height: f32) {
        println!("Window resized to: {}x{}", width as u32, height as u32);
    }
}

fn main() {
    let conf = create_test_window_conf();
    println!("Window config: {}x{}", conf.window_width, conf.window_height);

    miniquad::start(conf, || Box::new(WindowTest::new()));

    println!("\n=== Window Creation Test Completed ===");
}
