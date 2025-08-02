//! Scale showcase - demonstrate all available scales
//! Run with: cargo run --example scale_showcase

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

    let key = Key::new(Note::C, 4);

    println!("🎵 Scale Showcase - Exploring Musical Scales! 🎵\n");

    // === BASIC SCALES ===
    println!("🎼 BASIC SCALES:");

    println!("  Playing Major Scale (Ionian)...");
    let major = Melody::in_key(key).add_intervals(&interval::MAJOR_SCALE, duration::eighth_note());
    major.play(&sink, 44100);
    std::thread::sleep(Duration::from_millis(1000));

    println!("  Playing Natural Minor (Aeolian)...");
    let minor = Melody::in_key(key).add_intervals(&interval::MINOR_SCALE, duration::eighth_note());
    minor.play(&sink, 44100);
    std::thread::sleep(Duration::from_millis(1000));

    // === CHURCH MODES ===
    println!("\n⛪ CHURCH MODES:");

    println!("  Playing Dorian (jazzy minor)...");
    let dorian = Melody::in_key(key).add_intervals(&interval::DORIAN, duration::eighth_note());
    dorian.play(&sink, 44100);
    std::thread::sleep(Duration::from_millis(1000));

    println!("  Playing Mixolydian (bluesy major)...");
    let mixolydian =
        Melody::in_key(key).add_intervals(&interval::MIXOLYDIAN, duration::eighth_note());
    mixolydian.play(&sink, 44100);
    std::thread::sleep(Duration::from_millis(1000));

    println!("  Playing Lydian (dreamy major)...");
    let lydian = Melody::in_key(key).add_intervals(&interval::LYDIAN, duration::eighth_note());
    lydian.play(&sink, 44100);
    std::thread::sleep(Duration::from_millis(1000));

    // === BLUES SCALES ===
    println!("\n🎺 BLUES SCALES:");

    println!("  Playing Blues Minor...");
    let blues_minor =
        Melody::in_key(key).add_intervals(&interval::BLUES_MINOR, duration::eighth_note());
    blues_minor.play(&sink, 44100);
    std::thread::sleep(Duration::from_millis(900));

    println!("  Playing Pentatonic Blues...");
    let penta_blues =
        Melody::in_key(key).add_intervals(&interval::PENTATONIC_BLUES, duration::eighth_note());
    penta_blues.play(&sink, 44100);
    std::thread::sleep(Duration::from_millis(800));

    // === WORLD MUSIC ===
    println!("\n🌍 WORLD MUSIC SCALES:");

    println!("  Playing Japanese Hirajoshi...");
    let hirajoshi =
        Melody::in_key(key).add_intervals(&interval::JAPANESE_HIRAJOSHI, duration::eighth_note());
    hirajoshi.play(&sink, 44100);
    std::thread::sleep(Duration::from_millis(800));

    println!("  Playing Spanish Gypsy...");
    let spanish =
        Melody::in_key(key).add_intervals(&interval::SPANISH_GYPSY, duration::eighth_note());
    spanish.play(&sink, 44100);
    std::thread::sleep(Duration::from_millis(1000));

    println!("  Playing Arabic Maqam...");
    let arabic =
        Melody::in_key(key).add_intervals(&interval::ARABIC_MAQAM, duration::eighth_note());
    arabic.play(&sink, 44100);
    std::thread::sleep(Duration::from_millis(1000));

    // === EXOTIC SCALES ===
    println!("\n🔮 EXOTIC SCALES:");

    println!("  Playing Hungarian Minor (dramatic!)...");
    let hungarian =
        Melody::in_key(key).add_intervals(&interval::HUNGARIAN_MINOR, duration::eighth_note());
    hungarian.play(&sink, 44100);
    std::thread::sleep(Duration::from_millis(1000));

    println!("  Playing Enigmatic (mysterious!)...");
    let enigmatic =
        Melody::in_key(key).add_intervals(&interval::ENIGMATIC, duration::eighth_note());
    enigmatic.play(&sink, 44100);
    std::thread::sleep(Duration::from_millis(1000));

    // === SPECIAL SCALES ===
    println!("\n⚡ SPECIAL SCALES:");

    println!("  Playing Whole Tone (dreamy/floating)...");
    let whole_tone =
        Melody::in_key(key).add_intervals(&interval::WHOLE_TONE, duration::eighth_note());
    whole_tone.play(&sink, 44100);
    std::thread::sleep(Duration::from_millis(900));

    println!("  Playing Chromatic (all 12 notes)...");
    let chromatic =
        Melody::in_key(key).add_intervals(&interval::CHROMATIC, duration::sixteenth_note());
    chromatic.play(&sink, 44100);
    std::thread::sleep(Duration::from_millis(1200));

    println!("\n✨ Scale showcase complete! ✨");
    println!("🎼 You now have {} different scales to explore!", "30+"); // We added a lot of scales!
    println!("🎵 Try using different scales in your compositions for unique sounds!");

    Ok(())
}
