//! Melody composition and playback

use rodio::Sink;
use std::time::Duration;
use crate::{Key, MusicNote, Note, SquareWave};

/// Melody composer for playing sequences of notes
pub struct Melody {
    notes: Vec<MusicNote>,
    key: Option<Key>,
}

impl Melody {
    pub fn new() -> Self {
        Self {
            notes: Vec::new(),
            key: None,
        }
    }

    /// Create melody in a specific key
    pub fn in_key(key: Key) -> Self {
        Self {
            notes: Vec::new(),
            key: Some(key),
        }
    }

    /// Add note by absolute note and octave (original method)
    pub fn add_note(mut self, note: Note, octave: u8, duration: Duration) -> Self {
        self.notes.push(MusicNote::new(note, octave, duration));
        self
    }

    /// Add note by interval from the key's root (0 = root, 1 = one semitone up, etc.)
    pub fn add_interval(mut self, interval: i32, duration: Duration) -> Self {
        if let Some(key) = self.key {
            self.notes
                .push(MusicNote::from_key_interval(&key, interval, duration));
        } else {
            panic!(
                "Cannot add interval without setting a key first. Use Melody::in_key() or add_note() instead."
            );
        }
        self
    }

    /// Add multiple intervals at once
    pub fn add_intervals(mut self, intervals: &[i32], duration: Duration) -> Self {
        for &interval in intervals {
            self = self.add_interval(interval, duration);
        }
        self
    }

    /// Add a rest (silent note)
    pub fn add_rest(mut self, duration: Duration) -> Self {
        self.notes.push(MusicNote::new(Note::Rest, 0, duration));
        self
    }

    /// Set or change the key for subsequent interval additions
    pub fn set_key(mut self, key: Key) -> Self {
        self.key = Some(key);
        self
    }

    /// Play the melody using the provided sink
    pub fn play(&self, sink: &Sink, sample_rate: u32) {
        for note in &self.notes {
            let square_wave = SquareWave::from_note(note, sample_rate);
            sink.append(square_wave);
        }
    }
}

impl Default for Melody {
    fn default() -> Self {
        Self::new()
    }
}