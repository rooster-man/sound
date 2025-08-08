//! Audio generation and square wave synthesis

use crate::music::note::MusicNote;
use rodio::Source;
use std::f32::consts::TAU;
use std::time::Duration;
pub enum WaveType {
    Sine,
    Square,
    Triangle,
    Pulse,
    Sawtooth,
}

pub fn get_wave_type(wave_type: &str) -> WaveType {
    match wave_type.to_lowercase().as_str() {
        "sine" => WaveType::Sine,
        "square" => WaveType::Square,
        "triangle" => WaveType::Triangle,
        "pulse" => WaveType::Pulse,
        "sawtooth" => WaveType::Sawtooth,
        _ => WaveType::Sine,
    }
}

/// Square wave audio generator
pub struct Wave {
    wave_type: WaveType,
    sample_rate: u32,
    phase: f32,
    phase_step: f32,
    samples_played: usize,
    limit: Option<usize>,
}

impl Wave {
    pub fn finite(
        wave_type: WaveType,
        frequency: f32,
        sample_rate: u32,
        duration: Duration,
    ) -> Self {
        let period = sample_rate as f32 / frequency;
        let phase_step = 1.0f32 / period;
        let total_samples = (duration.as_secs_f32() * sample_rate as f32) as usize;
        Self {
            wave_type,
            sample_rate,
            phase: 0.0,
            phase_step,
            samples_played: 0,
            limit: Some(total_samples),
        }
    }

    pub fn infinite(wave_type: WaveType, frequency: f32, sample_rate: u32) -> Self {
        let period = sample_rate as f32 / frequency;
        let phase_step = 1.0f32 / period;
        Self {
            wave_type,
            sample_rate,
            phase: 0.0,
            phase_step,
            samples_played: 0,
            limit: None,
        }
    }

    pub fn from_note(wave_type: WaveType, note: &MusicNote, sample_rate: u32) -> Self {
        Self::finite(wave_type, note.frequency(), sample_rate, note.duration)
    }

    fn sine(&self) -> f32 {
        (TAU * self.phase).sin()
    }

    fn triangle(&self) -> f32 {
        4.0f32 * (self.phase - (self.phase + 0.5f32).floor()).abs() - 1f32
    }

    fn square(&self) -> f32 {
        if self.phase % 1.0f32 < 0.5f32 {
            1.0f32
        } else {
            -1.0f32
        }
    }

    fn pulse(&self) -> f32 {
        if self.phase % 1.0f32 < 0.25f32 {
            1.0f32
        } else {
            -1.0f32
        }
    }

    fn sawtooth(&self) -> f32 {
        2.0f32 * (self.phase - (self.phase + 0.5f32).floor())
    }
}

impl Iterator for Wave {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(limit) = self.limit {
            if self.samples_played >= limit {
                return None;
            }
        }

        let sample = match self.wave_type {
            WaveType::Sine => self.sine(),
            WaveType::Triangle => self.triangle(),
            WaveType::Square => self.square(),
            WaveType::Pulse => self.pulse(),
            WaveType::Sawtooth => self.sawtooth(),
        };

        self.phase = (self.phase + self.phase_step).rem_euclid(1.0f32);

        self.samples_played += 1;
        Some(sample)
    }
}

impl Source for Wave {
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
