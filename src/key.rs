//! Musical key system for interval-based composition

use crate::note::Note;

/// Represents a musical key with a root note and octave
#[derive(Debug, Clone, Copy)]
pub struct Key {
    pub root: Note,
    pub octave: u8,
}

impl Key {
    pub fn new(root: Note, octave: u8) -> Self {
        Self { root, octave }
    }

    /// Get a note by interval from the root (0 = root, 1 = one semitone up, etc.)
    pub fn note_at_interval(&self, interval: i32) -> (Note, u8) {
        if interval < 0 {
            return (Note::Rest, 0); // Negative intervals are rests
        }

        let root_semitone = self.root.to_semitone();
        let target_semitone = root_semitone + interval;

        // Calculate octave changes
        let octave_change = target_semitone / 12;
        let final_octave = (self.octave as i32 + octave_change) as u8;
        let final_note = Note::from_semitone(target_semitone);

        (final_note, final_octave)
    }
}