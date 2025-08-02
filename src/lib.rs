//! # Sound - Musical Note Composer with Key-Based Intervals
//!
//! A Rust library for generating and composing square wave music using musical intervals
//! and keys instead of absolute notes.
//!
//! ## Quick Start
//!
//! ```rust
//! use sound::{Key, Melody, Note, duration, interval};
//!
//! // Create a melody in C major using intervals
//! let c_major = Key::new(Note::C, 4);
//! let melody = Melody::in_key(c_major)
//!     .add_interval(interval::ROOT, duration::quarter_note())
//!     .add_interval(interval::MAJOR_THIRD, duration::quarter_note())
//!     .add_interval(interval::PERFECT_FIFTH, duration::quarter_note())
//!     .add_interval(interval::OCTAVE, duration::half_note());
//! ```

pub mod audio;
pub mod duration;
pub mod interval;
pub mod key;
pub mod melody;
pub mod note;

// Re-export main types for convenience
pub use audio::SquareWave;
pub use key::Key;
pub use melody::Melody;
pub use note::{MusicNote, Note};
