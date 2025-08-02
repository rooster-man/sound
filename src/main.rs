//! Demo application for the sound library
use rodio::{OutputStreamBuilder, Sink};
use std::env;
use std::time::Duration;

use sound::duration::duration;
use sound::interval::interval;
use sound::{Key, Melody, Note};

// Configuration struct for melody generation
#[derive(Debug)]
struct MelodyConfig {
    scale_name: String,
    scale_intervals: &'static [i32],
    note_positions: Vec<usize>,
    key: Key,
}

impl Default for MelodyConfig {
    fn default() -> Self {
        Self {
            scale_name: "major".to_string(),
            scale_intervals: &interval::MAJOR_SCALE,
            note_positions: vec![1, 2, 3, 4, 5, 6, 7, 8], // Default C major scale
            key: Key::new(Note::C, 4),
        }
    }
}

fn get_scale_by_name(name: &str) -> Result<(&'static [i32], String), String> {
    match name.to_lowercase().as_str() {
        // Basic scales
        "major" => Ok((&interval::MAJOR_SCALE, "Major".to_string())),
        "minor" => Ok((&interval::MINOR_SCALE, "Natural Minor".to_string())),

        // Pentatonic
        "pentatonic" | "penta" => Ok((&interval::PENTATONIC_MAJOR, "Pentatonic Major".to_string())),
        "pentatonic_minor" | "penta_minor" => {
            Ok((&interval::PENTATONIC_MINOR, "Pentatonic Minor".to_string()))
        }
        "blues" => Ok((&interval::BLUES_MINOR, "Blues Minor".to_string())),

        // Church modes
        "dorian" => Ok((&interval::DORIAN, "Dorian".to_string())),
        "phrygian" => Ok((&interval::PHRYGIAN, "Phrygian".to_string())),
        "lydian" => Ok((&interval::LYDIAN, "Lydian".to_string())),
        "mixolydian" => Ok((&interval::MIXOLYDIAN, "Mixolydian".to_string())),
        "locrian" => Ok((&interval::LOCRIAN, "Locrian".to_string())),

        // Exotic scales
        "harmonic_minor" | "harmonic" => {
            Ok((&interval::HARMONIC_MINOR, "Harmonic Minor".to_string()))
        }
        "hungarian" => Ok((&interval::HUNGARIAN_MINOR, "Hungarian Minor".to_string())),
        "japanese" => Ok((
            &interval::JAPANESE_HIRAJOSHI,
            "Japanese Hirajoshi".to_string(),
        )),
        "arabic" => Ok((&interval::ARABIC_MAQAM, "Arabic Maqam".to_string())),
        "spanish" => Ok((&interval::SPANISH_GYPSY, "Spanish Gypsy".to_string())),
        "whole_tone" | "wholetone" => Ok((&interval::WHOLE_TONE, "Whole Tone".to_string())),
        "chromatic" => Ok((&interval::CHROMATIC, "Chromatic".to_string())),

        _ => Err(format!(
            "Unknown scale: {}. Try: major, minor, dorian, blues, japanese, etc.",
            name
        )),
    }
}

fn parse_note_from_string(note_str: &str) -> Result<Note, String> {
    match note_str.to_uppercase().as_str() {
        "C" => Ok(Note::C),
        "CS" | "C#" => Ok(Note::Cs),
        "D" => Ok(Note::D),
        "DS" | "D#" => Ok(Note::Ds),
        "E" => Ok(Note::E),
        "F" => Ok(Note::F),
        "FS" | "F#" => Ok(Note::Fs),
        "G" => Ok(Note::G),
        "GS" | "G#" => Ok(Note::Gs),
        "A" => Ok(Note::A),
        "AS" | "A#" => Ok(Note::As),
        "B" => Ok(Note::B),
        _ => Err(format!("Unknown note: {}", note_str)),
    }
}

fn process_args(args: Vec<String>) -> Result<MelodyConfig, String> {
    println!("Args received: {:?}", args);

    if args.len() < 2 {
        return Err("Usage: program \"scale:major|key:C|notes:1,2,3,4,5\"".to_string());
    }

    let mut config = MelodyConfig::default();
    let parts: Vec<String> = args[1].split("|").map(|s| s.trim().to_string()).collect();
    println!("Parsed parts: {:?}", parts);

    for part in parts {
        if part.starts_with("scale:") {
            let scale_name = &part[6..];
            let (scale_intervals, full_name) = get_scale_by_name(scale_name)?;
            config.scale_name = full_name;
            config.scale_intervals = scale_intervals;
        } else if part.starts_with("key:") {
            let key_name = &part[4..];
            let note = parse_note_from_string(key_name)?;
            config.key = Key::new(note, 4);
        } else if part.starts_with("notes:") {
            let notes_str = &part[6..];
            let positions: Result<Vec<usize>, _> = notes_str
                .split(",")
                .map(|s| s.trim().parse::<usize>())
                .collect();
            config.note_positions =
                positions.map_err(|e| format!("Invalid note positions: {}", e))?;
        } else {
            // Fallback: treat as comma-separated note positions
            let positions: Result<Vec<usize>, _> =
                part.split(",").map(|s| s.trim().parse::<usize>()).collect();
            if let Ok(pos) = positions {
                config.note_positions = pos;
            }
        }
    }

    Ok(config)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Set up audio output
    let stream_handle =
        OutputStreamBuilder::open_default_stream().expect("Failed to open audio stream");
    let sink = Sink::connect_new(&stream_handle.mixer());

    let sample_rate = 44100;

    // get cli args
    let args: Vec<String> = env::args().collect();

    // Process command line arguments if provided
    match process_args(args) {
        Ok(config) => {
            println!("✅ Successfully parsed melody configuration:");
            println!("  🎼 Scale: {}", config.scale_name);
            println!("  🎹 Key: {:?}", config.key.root);
            println!("  🎵 Note positions: {:?}", config.note_positions);

            // Create and play the custom melody
            println!("\n🎶 Playing your custom melody...");
            let mut melody = Melody::in_key(config.key);

            for &position in &config.note_positions {
                if position == 0 || position > config.scale_intervals.len() {
                    println!(
                        "⚠️  Warning: Note position {} is out of range for this scale",
                        position
                    );
                    continue;
                }
                let interval = config.scale_intervals[position - 1];
                melody = melody.add_interval(interval, duration::quarter_note());
            }

            melody.play(&sink, sample_rate);
            let duration_ms = config.note_positions.len() as u64 * 300;
            std::thread::sleep(Duration::from_millis(duration_ms));

            println!("✨ Custom melody complete!");

            return Ok(()); // Exit early, don't run default demo
        }
        Err(error) => {
            println!("⚠️  Argument parsing error: {}", error);
            println!("🎵 Running default demo instead...");
            println!("💡 Examples:");
            println!("   cargo run \"scale:major|notes:1,2,3,4,5,6,7,8\"");
            println!("   cargo run \"scale:minor|key:A|notes:1,3,5,8\"");
            println!("   cargo run \"scale:dorian|key:D|notes:1,2,3,5,6\"");
            println!("   cargo run \"scale:japanese|notes:1,2,3,4,5\"");
        }
    }

    Ok(())
}
