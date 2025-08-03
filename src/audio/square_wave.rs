//! Audio generation and square wave synthesis

use crate::music::note::MusicNote;
use rodio::Source;
use std::time::Duration;

/// Square wave audio generator
pub struct SquareWave {
    frequency: f32,
    sample_rate: u32,
    phase: f32,
    duration: Duration,
    samples_played: usize,
    total_samples: usize,
}

impl SquareWave {
    pub fn new(frequency: f32, sample_rate: u32, duration: Duration) -> Self {
        let total_samples = (duration.as_secs_f32() * sample_rate as f32) as usize;
        Self {
            frequency,
            sample_rate,
            phase: 0.0,
            duration,
            samples_played: 0,
            total_samples,
        }
    }

    pub fn from_note(note: &MusicNote, sample_rate: u32) -> Self {
        Self::new(note.frequency(), sample_rate, note.duration)
    }
}

impl Iterator for SquareWave {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.samples_played >= self.total_samples {
            return None; // Note duration completed
        }

        let sample = if self.frequency == 0.0 {
            // Rest note - silence
            0.0
        } else if self.phase < 0.5 {
            0.1
        } else {
            -0.1
        };

        self.phase += self.frequency / self.sample_rate as f32;
        if self.phase >= 1.0 {
            self.phase -= 1.0;
        }

        self.samples_played += 1;
        Some(sample)
    }
}

impl Source for SquareWave {
    fn current_span_len(&self) -> Option<usize> {
        Some(self.total_samples - self.samples_played)
    }

    fn channels(&self) -> u16 {
        1
    }

    fn sample_rate(&self) -> u32 {
        self.sample_rate
    }

    fn total_duration(&self) -> Option<Duration> {
        Some(self.duration)
    }
}
