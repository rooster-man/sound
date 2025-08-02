//! Simple melody example using the sound library
//! Run with: cargo run --example simple_melody

use rodio::{OutputStreamBuilder, Sink};
use std::time::Duration;

use sound::{Key, Melody, Note};
use sound::duration::duration;
use sound::interval::interval;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Set up audio output
    let stream_handle = OutputStreamBuilder::open_default_stream()
        .expect("Failed to open audio stream");
    let sink = Sink::connect_new(&stream_handle.mixer());

    println!("🎵 Playing a simple melody in different keys...\n");

    // Define a simple melody pattern using intervals
    let melody_pattern = |key: Key| {
        Melody::in_key(key)
            .add_interval(interval::ROOT, duration::quarter_note())
            .add_interval(interval::MAJOR_THIRD, duration::quarter_note())
            .add_interval(interval::PERFECT_FIFTH, duration::quarter_note())
            .add_interval(interval::OCTAVE, duration::quarter_note())
            .add_rest(duration::eighth_note())
            .add_interval(interval::PERFECT_FIFTH, duration::quarter_note())
            .add_interval(interval::MAJOR_THIRD, duration::quarter_note())
            .add_interval(interval::ROOT, duration::half_note())
    };

    // Play the same melody in different keys
    let keys = [
        (Key::new(Note::C, 4), "C major"),
        (Key::new(Note::F, 4), "F major"),
        (Key::new(Note::G, 4), "G major"),
    ];

    for (key, name) in keys.iter() {
        println!("Playing in {}...", name);
        let melody = melody_pattern(*key);
        melody.play(&sink, 44100);
        std::thread::sleep(Duration::from_millis(2000));
    }

    println!("\n✨ Notice how the same interval pattern sounds different in each key!");
    println!("🎼 This is the power of interval-based composition!");

    Ok(())
}