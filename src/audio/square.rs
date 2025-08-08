//! Audio generation and square wave synthesis

use crate::music::note::MusicNote;
use rodio::Source;
use std::time::Duration;

/// Square wave audio generator
pub struct Square {
    frequency: f32,
    sample_rate: u32,
    phase: f32,
    phase_step: f32,
    period: f32,
    samples_played: usize,
    limit: Option<usize>,
}

impl Square {
    pub fn finite(frequency: f32, sample_rate: u32, duration: Duration) -> Self {
        let period = sample_rate as f32 / frequency;
        let phase_step = 1.0f32 / period;
        let total_samples = (duration.as_secs_f32() * sample_rate as f32) as usize;
        Self {
            frequency,
            sample_rate,
            phase: 0.0,
            phase_step,
            period,
            samples_played: 0,
            limit: Some(total_samples),
        }
    }

    pub fn infinite(frequency: f32, sample_rate: u32) -> Self {
        let period = sample_rate as f32 / frequency;
        let phase_step = 1.0f32 / period;
        Self {
            frequency,
            sample_rate,
            phase: 0.0,
            phase_step,
            period,
            samples_played: 0,
            limit: None,
        }
    }

    pub fn from_note(note: &MusicNote, sample_rate: u32) -> Self {
        Self::finite(note.frequency(), sample_rate, note.duration)
    }

    fn wave_function(&self, phase: f32) -> f32 {
        if phase % 1.0f32 < 0.5f32 {
            1.0f32
        } else {
            -1.0f32
        }
    }
}

impl Iterator for Square {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(limit) = self.limit {
            if self.samples_played >= limit {
                return None;
            }
        }

        let sample = if self.phase % 1.0f32 < 0.5f32 {
            1.0f32
        } else {
            -1.0f32
        };

        self.phase = (self.phase + self.phase_step).rem_euclid(1.0f32);

        self.samples_played += 1;
        Some(sample)
    }
}

impl Source for Square {
    fn current_span_len(&self) -> Option<usize> {
        self.limit.map(|lim| lim - self.samples_played)
    }

    fn channels(&self) -> u16 {
        1
    }

    fn sample_rate(&self) -> u32 {
        self.sample_rate
    }

    fn total_duration(&self) -> Option<Duration> {
        self.limit
            .map(|lim| Duration::from_secs_f32(lim as f32 / self.sample_rate as f32))
    }
}
