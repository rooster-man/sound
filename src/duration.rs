//! Duration utilities for musical timing

use std::time::Duration;

/// Helper functions to create common note durations
pub mod duration {
    use super::Duration;

    pub fn whole_note() -> Duration {
        Duration::from_millis(1000)
    }

    pub fn half_note() -> Duration {
        Duration::from_millis(500)
    }

    pub fn quarter_note() -> Duration {
        Duration::from_millis(250)
    }

    pub fn eighth_note() -> Duration {
        Duration::from_millis(125)
    }

    pub fn sixteenth_note() -> Duration {
        Duration::from_millis(63)
    }
}