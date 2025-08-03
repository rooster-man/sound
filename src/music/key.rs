//! Musical key system for interval-based composition

use super::note::Note;

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
    /// Now properly handles negative intervals for lower octaves
    pub fn note_at_interval(&self, interval: i32) -> (Note, u8) {
        let root_semitone = self.root.to_semitone() as i32;
        let target_semitone = root_semitone + interval;

        // Calculate octave changes (handle negative properly)
        let octave_change = if target_semitone >= 0 {
            target_semitone / 12
        } else {
            // For negative semitones, we need to go down octaves
            (target_semitone - 11) / 12 // This ensures proper floor division
        };

        // Calculate final octave, but don't let it go below 0
        let final_octave = ((self.octave as i32 + octave_change).max(0)) as u8;

        // Calculate the note (handle negative modulo properly)
        let note_semitone = if target_semitone >= 0 {
            target_semitone % 12
        } else {
            ((target_semitone % 12) + 12) % 12 // Positive modulo for negative numbers
        };

        let final_note = Note::from_semitone(note_semitone);

        (final_note, final_octave)
    }
}
