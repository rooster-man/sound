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
pub mod cli;
// pub mod interval;
// pub mod key;
// pub mod melody;
pub mod music;
// pub mod note;

// Re-export main types for convenience
pub use audio::square_wave::SquareWave;
pub use music::key::Key;
pub use music::melody::{Melody, MelodyConfig, NoteElement};
pub use music::note::{MusicNote, Note};
pub use music::util::{get_scale_by_name, parse_note_from_string, parse_note_notation};
