//! Shared utilities for miniquad tests
//!
//! This module provides common functions and helpers used across multiple test programs,
//! including window setup, event handling utilities, and test instrumentation.

use miniquad::conf;

/// Window configuration for test programs
/// 
/// Provides sensible defaults for creating test windows:
/// - 800x600 resolution
/// - 60 FPS target
/// - "Miniquad Test" window title
pub fn create_test_window_conf() -> conf::Conf {
    conf::Conf {
        window_title: "Miniquad Test".to_string(),
        window_width: 800,
        window_height: 600,
        ..Default::default()
    }
}

/// A simple event logger for debugging
/// 
/// Prints miniquad events to stdout for inspection during manual testing.
pub struct EventLogger;

impl EventLogger {
    pub fn new() -> Self {
        EventLogger
    }

    pub fn log(&self, msg: impl std::fmt::Display) {
        println!("[EVENT] {}", msg);
    }
}

impl Default for EventLogger {
    fn default() -> Self {
        Self::new()
    }
}

/// Helper to measure frame timing
/// 
/// Useful for checking if your test program maintains consistent frame rates.
pub struct FrameTimer {
    frame_count: u64,
    total_duration: std::time::Duration,
    last_frame_time: std::time::Instant,
}

impl FrameTimer {
    pub fn new() -> Self {
        FrameTimer {
            frame_count: 0,
            total_duration: std::time::Duration::ZERO,
            last_frame_time: std::time::Instant::now(),
        }
    }

    /// Record a frame and return the elapsed time since last call
    pub fn tick(&mut self) -> std::time::Duration {
        let elapsed = self.last_frame_time.elapsed();
        self.total_duration += elapsed;
        self.frame_count += 1;
        self.last_frame_time = std::time::Instant::now();
        elapsed
    }

    /// Get average frame time
    pub fn average_frame_time(&self) -> std::time::Duration {
        if self.frame_count == 0 {
            std::time::Duration::ZERO
        } else {
            self.total_duration / self.frame_count as u32
        }
    }

    /// Get average frames per second
    pub fn average_fps(&self) -> f64 {
        let avg_frame_time = self.average_frame_time();
        if avg_frame_time.as_secs_f64() > 0.0 {
            1.0 / avg_frame_time.as_secs_f64()
        } else {
            0.0
        }
    }

    /// Get total frame count
    pub fn frame_count(&self) -> u64 {
        self.frame_count
    }
}

impl Default for FrameTimer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn window_conf_has_reasonable_defaults() {
        let conf = create_test_window_conf();
        assert_eq!(conf.window_width, 800);
        assert_eq!(conf.window_height, 600);
        assert!(conf.window_title.contains("Miniquad"));
    }

    #[test]
    fn frame_timer_tracks_frames() {
        let mut timer = FrameTimer::new();
        assert_eq!(timer.frame_count(), 0);

        timer.tick();
        assert_eq!(timer.frame_count(), 1);

        timer.tick();
        assert_eq!(timer.frame_count(), 2);
    }
}
