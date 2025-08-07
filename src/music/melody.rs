//! Melody composition and playback

use crate::{music::interval, Key, MusicNote, Note, Square};
use rodio::Sink;
use std::time::Duration;

/// Melody composer for playing sequences of notes
pub struct Melody {
    notes: Vec<MusicNote>,
    key: Key,
    sample_rate: u32,
    pub bpm: u32,
    pub base_note_duration: Duration,
    sixteenth_note_duration: Duration,
    pub scale_name: String,
    pub scale_intervals: &'static [i32],
    pub note_elements: Vec<NoteElement>,
    pub should_loop: bool,
    pub base_duration: String,
}

impl Melody {
    pub fn new(config: MelodyConfig) -> Self {
        let mut melody = Self {
            notes: Vec::new(),
            key: config.key,
            sample_rate: config.sample_rate,
            bpm: config.bpm,
            base_duration: config.base_duration,
            scale_name: config.scale_name,
            scale_intervals: config.scale_intervals,
            note_elements: config.note_elements,
            should_loop: config.should_loop,
            base_note_duration: Duration::from_millis(0),
            sixteenth_note_duration: Duration::from_millis(0),
        };

        let (base_note_duration, sixteenth_note_duration) =
            Melody::calculate_durations(melody.bpm, &melody.base_duration);
        melody.base_note_duration = base_note_duration;
        melody.sixteenth_note_duration = sixteenth_note_duration;

        // Build the melody once
        // let mut melody = self;
        let mut i: usize = 0;

        while i < melody.note_elements.len() {
            match &melody.note_elements[i] {
                NoteElement::Note(position, octave_offset) => {
                    if *position == 0 || *position > melody.scale_intervals.len() {
                        println!(
                            "⚠️  Warning: Note position {} is out of range for this scale",
                            position
                        );
                        i += 1;
                        continue;
                    }
                    // Calculate interval with octave offset (12 semitones per octave)
                    let base_interval = config.scale_intervals[position - 1];
                    let interval = base_interval + (octave_offset * 12);

                    // Count sustains that follow this note
                    let mut sustain_count = 0;
                    let mut j = i + 1;
                    while j < melody.note_elements.len() {
                        match &melody.note_elements[j] {
                            NoteElement::Sustain => {
                                sustain_count += 1;
                                j += 1;
                            }
                            _ => break,
                        }
                    }

                    // Calculate duration: configurable base duration + sustains (sixteenth notes)
                    let base_duration = base_note_duration;
                    let sustain_duration = Duration::from_millis(
                        sixteenth_note_duration.as_millis() as u64 * sustain_count as u64,
                    );
                    let total_duration = base_duration + sustain_duration;

                    melody = melody.add_interval(interval, total_duration);

                    // Skip past the sustains we just processed
                    i = j;
                }
                NoteElement::Rest => {
                    melody = melody.add_rest(base_note_duration);
                    i += 1;
                }
                NoteElement::Sustain => {
                    // Sustains without a preceding note are treated as rests
                    melody = melody.add_rest(base_note_duration);
                    i += 1;
                }
            }
        }
        melody
    }

    // /// Create melody in a specific key
    // pub fn in_key(key: Key) -> Self {
    //     Self {
    //         notes: Vec::new(),
    //         key: Some(key),
    //     }
    // }

    /// Add note by absolute note and octave (original method)
    pub fn add_note(mut self, note: Note, octave: u8, duration: Duration) -> Self {
        self.notes.push(MusicNote::new(note, octave, duration));
        self
    }

    /// Add note by interval from the key's root (0 = root, 1 = one semitone up, etc.)
    pub fn add_interval(mut self, interval: i32, duration: Duration) -> Self {
        // if let Some(key) = self.key {
        //     self.notes
        //         .push(MusicNote::from_key_interval(&key, interval, duration));
        // } else {
        //     panic!(
        //         "Cannot add interval without setting a key first. Use Melody::in_key() or add_note() instead."
        //     );
        // }
        self.notes
            .push(MusicNote::from_key_interval(&self.key, interval, duration));
        self
    }

    /// Add multiple intervals at once
    // pub fn add_intervals(mut self, intervals: &[i32], duration: Duration) -> Self {
    //     for &interval in intervals {
    //         self = self.add_interval(interval, duration);
    //     }
    //     self
    // }

    /// Add a rest (silent note)
    pub fn add_rest(mut self, duration: Duration) -> Self {
        self.notes.push(MusicNote::new(Note::Rest, 0, duration));
        self
    }

    /// Set or change the key for subsequent interval additions
    pub fn set_key(mut self, key: Key) -> Self {
        self.key = key;
        self
    }

    /// Play the melody using the provided sink
    pub fn play(&self, sink: &Sink) {
        for note in &self.notes {
            let square_wave = Square::from_note(note, self.sample_rate);
            sink.append(square_wave);
        }
    }

    fn calculate_durations(bpm: u32, base_duration: &str) -> (Duration, Duration) {
        let quarter_note_ms = 60_000 / bpm; // milliseconds per quarter note
        let sixteenth_note_ms = quarter_note_ms / 4; // sixteenth note for sustains

        // Calculate base duration based on the specified type
        let base_duration_ms = match base_duration.to_lowercase().as_str() {
            "whole" | "1" => quarter_note_ms * 4,
            "half" | "2" => quarter_note_ms * 2,
            "quarter" | "4" => quarter_note_ms,
            "eighth" | "8" => quarter_note_ms / 2,
            "sixteenth" | "16" => quarter_note_ms / 4,
            _ => quarter_note_ms / 4, // fallback to sixteenth
        };

        (
            Duration::from_millis(base_duration_ms as u64), // configurable base duration
            Duration::from_millis(sixteenth_note_ms as u64), // sixteenth note for sustains
        )
    }
}

// impl Default for Melody {
//     fn default() -> Self {
//         Self::new()
//     }
// }

/// Represents different musical elements in our enhanced notation
#[derive(Debug, Clone)]
pub enum NoteElement {
    /// A note at a specific scale position with octave offset
    Note(usize, i32), // (scale_position, octave_offset)
    /// A sixteenth-note rest
    Rest,
    /// A sixteenth-note sustain (extends the previous note)
    Sustain,
}

// Configuration struct for melody generation
#[derive(Debug)]
pub struct MelodyConfig {
    pub scale_name: String,
    pub scale_intervals: &'static [i32],
    pub note_elements: Vec<NoteElement>,
    pub key: Key,
    pub bpm: u32,
    pub should_loop: bool,
    pub base_duration: String,
    pub sample_rate: u32,
}

impl Default for MelodyConfig {
    fn default() -> Self {
        Self {
            scale_name: "major".to_string(),
            scale_intervals: &interval::MAJOR_SCALE,
            note_elements: vec![
                NoteElement::Note(1, 0),
                NoteElement::Note(2, 0),
                NoteElement::Note(3, 0),
                NoteElement::Note(4, 0),
                NoteElement::Note(5, 0),
                NoteElement::Note(6, 0),
                NoteElement::Note(7, 0),
                NoteElement::Note(8, 0),
            ], // Default C major scale
            key: Key::new(Note::C, 4),
            bpm: 120,
            should_loop: false,
            base_duration: "sixteenth".to_string(),
            sample_rate: 44100,
        }
    }
}
