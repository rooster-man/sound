# 🎵 Musical Note Composer with Key-Based Intervals

A Rust library for generating and composing square wave music using musical intervals and keys instead of absolute notes.

## Key Features

### 🎹 Key-Based Composition

- Define a musical key (root note + octave)
- Add notes using intervals (0 = root, 7 = perfect fifth, etc.)
- Automatic transposition to any key

### 🎼 Musical Intervals

```rust
// Instead of absolute notes:
.add_note(Note::C, 4, duration)
.add_note(Note::E, 4, duration)
.add_note(Note::G, 4, duration)

// Use intervals (works in any key!):
.add_interval(interval::ROOT, duration)
.add_interval(interval::MAJOR_THIRD, duration)
.add_interval(interval::PERFECT_FIFTH, duration)
```

### 🎯 Pre-defined Musical Patterns

- **Scales**: Major, Minor, Pentatonic
- **Chords**: Major triad, Minor triad, Diminished, Augmented
- **Intervals**: All standard musical intervals with names

## Quick Start

```rust
use sound::{Key, Melody, Note, duration, interval};

// Create a melody in C major
let c_major = Key::new(Note::C, 4);
let melody = Melody::in_key(c_major)
    .add_interval(interval::ROOT, duration::quarter_note())
    .add_interval(interval::MAJOR_THIRD, duration::quarter_note())
    .add_interval(interval::PERFECT_FIFTH, duration::quarter_note())
    .add_interval(interval::OCTAVE, duration::half_note());

// Play the same melody in F major (automatic transposition!)
let f_major = Key::new(Note::F, 4);
let transposed = Melody::in_key(f_major)
    .add_interval(interval::ROOT, duration::quarter_note())
    .add_interval(interval::MAJOR_THIRD, duration::quarter_note())
    .add_interval(interval::PERFECT_FIFTH, duration::quarter_note())
    .add_interval(interval::OCTAVE, duration::half_note());
```

## Available Intervals

### Basic Intervals

- `interval::ROOT` (0) - Root note
- `interval::MAJOR_SECOND` (2) - Whole step up
- `interval::MAJOR_THIRD` (4) - Major third
- `interval::PERFECT_FOURTH` (5) - Perfect fourth
- `interval::PERFECT_FIFTH` (7) - Perfect fifth
- `interval::MAJOR_SIXTH` (9) - Major sixth
- `interval::MAJOR_SEVENTH` (11) - Major seventh
- `interval::OCTAVE` (12) - One octave up

### Scale Patterns

- `interval::MAJOR_SCALE` - Do, Re, Mi, Fa, Sol, La, Ti, Do
- `interval::MINOR_SCALE` - Natural minor scale
- `interval::PENTATONIC_MAJOR` - 5-note major scale
- `interval::PENTATONIC_MINOR` - 5-note minor scale

### Chord Patterns

- `interval::MAJOR_TRIAD` - [0, 4, 7] (Root, Major 3rd, Perfect 5th)
- `interval::MINOR_TRIAD` - [0, 3, 7] (Root, Minor 3rd, Perfect 5th)
- `interval::DIMINISHED_TRIAD` - [0, 3, 6]
- `interval::AUGMENTED_TRIAD` - [0, 4, 8]

## Examples

### Simple Scale

```rust
// C major scale using intervals
let scale = Melody::in_key(Key::new(Note::C, 4))
    .add_intervals(&interval::MAJOR_SCALE, duration::quarter_note());
```

### Chord Progression

```rust
// I-vi-IV-V progression
let progression = Melody::new()
    .set_key(Key::new(Note::C, 4))  // I (C major)
    .add_intervals(&interval::MAJOR_TRIAD, duration::quarter_note())
    .set_key(Key::new(Note::A, 3))  // vi (A minor)
    .add_intervals(&interval::MINOR_TRIAD, duration::quarter_note())
    .set_key(Key::new(Note::F, 4))  // IV (F major)
    .add_intervals(&interval::MAJOR_TRIAD, duration::quarter_note())
    .set_key(Key::new(Note::G, 4))  // V (G major)
    .add_intervals(&interval::MAJOR_TRIAD, duration::quarter_note());
```

### Melody with Key Changes

```rust
let melody = Melody::in_key(Key::new(Note::C, 4))
    .add_interval(interval::ROOT, duration::quarter_note())
    .add_interval(interval::MAJOR_THIRD, duration::quarter_note())
    .set_key(Key::new(Note::F, 4))  // Modulate to F major
    .add_interval(interval::ROOT, duration::quarter_note())
    .add_interval(interval::MAJOR_THIRD, duration::quarter_note());
```

## Why Use Intervals?

### 🎼 Musical Thinking

Musicians think in terms of relationships between notes, not absolute pitches. A melody is remembered as "up a fifth, down a third" rather than "C to G to E".

### 🎯 Easy Transposition

Write a melody once, play it in any key instantly. Perfect for creating variations or accommodating different vocal ranges.

### 🏗️ Reusable Patterns

Define common musical patterns (scales, chords, progressions) once and reuse them in any key.

### 🎵 Harmonic Relationships

Intervals preserve the harmonic relationships that make music sound "right". A major chord sounds major in any key.

## Running the Examples

```bash
# Run the main demo
cargo run

# See interval examples in action
cargo run --example intervals

# Basic usage examples
cargo run --example basic
```

## Note Durations

```rust
duration::whole_note()      // 1000ms
duration::half_note()       // 500ms
duration::quarter_note()    // 250ms
duration::eighth_note()     // 125ms
duration::sixteenth_note()  // 63ms

// Or custom durations
Duration::from_millis(300)
```

## Project Structure

The library is organized into focused modules for maintainability:

```
src/
├── lib.rs          # Main library interface & exports
├── note.rs         # Note enum & musical note utilities
├── key.rs          # Key system for interval-based composition
├── audio.rs        # SquareWave audio generation
├── melody.rs       # Melody composition & playback
├── duration.rs     # Duration helper functions
├── interval.rs     # Musical interval constants & patterns
└── main.rs         # Demo application
```

### Module Overview

- **`note`**: `Note` enum, `MusicNote` struct, frequency calculations
- **`key`**: `Key` struct for interval-based composition
- **`audio`**: `SquareWave` generator implementing rodio's `Source` trait
- **`melody`**: `Melody` composer with interval and key support
- **`duration`**: Helper functions for common note durations
- **`interval`**: Constants for musical intervals, scales, and chords

### Using as a Library

```rust
use sound::{Key, Melody, Note};
use sound::duration::duration;
use sound::interval::interval;

let melody = Melody::in_key(Key::new(Note::C, 4))
    .add_intervals(&interval::MAJOR_SCALE, duration::quarter_note());
```

## Architecture

The system automatically handles:

- Octave boundaries (interval 12+ cross octave boundaries)
- Key transposition (same pattern, any key)
- Note timing and sequencing
- Audio sample generation and playback

## Future Enhancements

- Multiple waveforms (sine, triangle, sawtooth)
- Polyphonic playback (chords)
- Effects (reverb, delay, filter)
- MIDI file export
- Real-time playback control
- More complex rhythmic patterns

## Musical Theory Integration

This library aligns with standard music theory:

- 12-tone equal temperament tuning
- Standard frequency relationships (A4 = 440Hz)
- Chromatic semitone intervals
- Traditional chord and scale patterns
- Common notation conventions

Perfect for:

- Learning music theory through code
- Algorithmic composition
- Game audio programming
- Educational music applications
- Rapid prototyping of musical ideas

lotr example:
cargo run -- 123---5---3---2---1-------3---5---6----8-7-5----3-----432----
