//! Demo application for the sound library

use rodio::{OutputStreamBuilder, Sink};
use std::time::Duration;

use sound::duration::duration;
use sound::interval::interval;
use sound::{Key, Melody, Note};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Set up audio output
    let stream_handle =
        OutputStreamBuilder::open_default_stream().expect("Failed to open audio stream");
    let sink = Sink::connect_new(&stream_handle.mixer());

    let sample_rate = 44100;

    println!("🎹 Key-Based Musical Composer Demo 🎹\n");

    // Example 1: Major scale using intervals in C major
    println!("Playing C major scale using intervals...");
    let c_major_key = Key::new(Note::C, 4);
    let c_major_scale =
        Melody::in_key(c_major_key).add_intervals(&interval::MAJOR_SCALE, duration::quarter_note());

    c_major_scale.play(&sink, sample_rate);
    std::thread::sleep(Duration::from_millis(2200));

    // Example 2: Same scale pattern in different key (F major)
    println!("Playing F major scale using the same intervals...");
    let f_major_key = Key::new(Note::F, 4);
    let f_major_scale =
        Melody::in_key(f_major_key).add_intervals(&interval::MAJOR_SCALE, duration::quarter_note());

    f_major_scale.play(&sink, sample_rate);
    std::thread::sleep(Duration::from_millis(2200));

    // Example 3: Minor scale in A minor
    println!("Playing A minor scale...");
    let a_minor_key = Key::new(Note::A, 4);
    let a_minor_scale =
        Melody::in_key(a_minor_key).add_intervals(&interval::MINOR_SCALE, duration::quarter_note());

    a_minor_scale.play(&sink, sample_rate);
    std::thread::sleep(Duration::from_millis(2200));

    // Example 4: Chord progression using intervals
    println!("Playing chord progression using intervals...");
    let chord_progression = Melody::in_key(Key::new(Note::C, 4))
        // C major chord (0, 4, 7)
        .add_intervals(&interval::MAJOR_TRIAD, duration::eighth_note())
        .add_rest(duration::eighth_note())
        // F major chord - switch to F and play major triad
        .set_key(Key::new(Note::F, 4))
        .add_intervals(&interval::MAJOR_TRIAD, duration::eighth_note())
        .add_rest(duration::eighth_note())
        // G major chord
        .set_key(Key::new(Note::G, 4))
        .add_intervals(&interval::MAJOR_TRIAD, duration::eighth_note())
        .add_rest(duration::eighth_note())
        // Back to C major
        .set_key(Key::new(Note::C, 4))
        .add_interval(interval::ROOT, duration::quarter_note())
        .add_interval(interval::MAJOR_THIRD, duration::quarter_note())
        .add_interval(interval::PERFECT_FIFTH, duration::half_note());

    chord_progression.play(&sink, sample_rate);
    std::thread::sleep(Duration::from_millis(3000));

    // Example 5: Simple melody using individual intervals
    println!("Playing simple melody using individual intervals...");
    let simple_melody = Melody::in_key(Key::new(Note::C, 4))
        .add_interval(interval::ROOT, duration::quarter_note()) // C
        .add_interval(interval::MAJOR_THIRD, duration::quarter_note()) // E
        .add_interval(interval::PERFECT_FIFTH, duration::quarter_note()) // G
        .add_interval(interval::OCTAVE, duration::quarter_note()) // C (higher)
        .add_rest(duration::quarter_note())
        .add_interval(interval::MAJOR_SEVENTH, duration::quarter_note()) // B
        .add_interval(interval::PERFECT_FIFTH, duration::quarter_note()) // G
        .add_interval(interval::MAJOR_THIRD, duration::quarter_note()) // E
        .add_interval(interval::ROOT, duration::half_note()); // C

    simple_melody.play(&sink, sample_rate);
    std::thread::sleep(Duration::from_millis(2500));

    // Example 6: Pentatonic scale
    println!("Playing C major pentatonic scale...");
    let pentatonic = Melody::in_key(Key::new(Note::C, 4))
        .add_intervals(&interval::PENTATONIC_MAJOR, duration::quarter_note());

    pentatonic.play(&sink, sample_rate);
    std::thread::sleep(Duration::from_millis(1800));

    println!("\n✨ Key-based demo complete! ✨");
    println!("🎼 Now you can compose in any key using intervals!");
    println!("🎵 Try different keys and the same interval patterns will transpose automatically!");

    Ok(())
}
