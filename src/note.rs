//! Musical note definitions and utilities

use crate::Key;
use std::time::Duration;

/// Standard musical note frequencies (in Hz) for octave 4
#[derive(Debug, Clone, Copy)]
pub enum Note {
    C = 261,  // C4
    Cs = 277, // C#4
    D = 294,  // D4
    Ds = 311, // D#4
    E = 330,  // E4
    F = 349,  // F4
    Fs = 370, // F#4
    G = 392,  // G4
    Gs = 415, // G#4
    A = 440,  // A4
    As = 466, // A#4
    B = 494,  // B4
    Rest = 0, // Silent note
}

impl Note {
    /// Get frequency for a specific octave (0-8)
    pub fn frequency(&self, octave: u8) -> f32 {
        let base_freq = *self as u32 as f32;
        if base_freq == 0.0 {
            return 0.0; // Rest note
        }

        // Octave 4 is our base, adjust accordingly
        let octave_multiplier = 2.0_f32.powi(octave as i32 - 4);
        base_freq * octave_multiplier
    }

    /// Convert note to semitone number (C = 0, C# = 1, D = 2, etc.)
    pub fn to_semitone(&self) -> i32 {
        match self {
            Note::C => 0,
            Note::Cs => 1,
            Note::D => 2,
            Note::Ds => 3,
            Note::E => 4,
            Note::F => 5,
            Note::Fs => 6,
            Note::G => 7,
            Note::Gs => 8,
            Note::A => 9,
            Note::As => 10,
            Note::B => 11,
            Note::Rest => -1, // Special case
        }
    }

    /// Create note from semitone number (0-11)
    pub fn from_semitone(semitone: i32) -> Self {
        match semitone % 12 {
            0 => Note::C,
            1 => Note::Cs,
            2 => Note::D,
            3 => Note::Ds,
            4 => Note::E,
            5 => Note::F,
            6 => Note::Fs,
            7 => Note::G,
            8 => Note::Gs,
            9 => Note::A,
            10 => Note::As,
            11 => Note::B,
            _ => Note::C, // fallback
        }
    }
}

/// Represents a musical note with duration
#[derive(Debug, Clone)]
pub struct MusicNote {
    pub note: Note,
    pub octave: u8,
    pub duration: Duration,
}

impl MusicNote {
    pub fn new(note: Note, octave: u8, duration: Duration) -> Self {
        Self {
            note,
            octave,
            duration,
        }
    }

    /// Create a note from a key and interval
    pub fn from_key_interval(key: &Key, interval: i32, duration: Duration) -> Self {
        let (note, octave) = key.note_at_interval(interval);
        Self::new(note, octave, duration)
    }

    pub fn frequency(&self) -> f32 {
        self.note.frequency(self.octave)
    }
}
