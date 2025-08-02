//! Demo application for the sound library
use clap::Parser;
use rodio::{OutputStreamBuilder, Sink};
use std::time::Duration;

use sound::duration::duration;
use sound::interval::interval;
use sound::{Key, Melody, Note};

/// Sound CLI - Generate and play melodies with customizable scales and keys
#[derive(Parser, Debug)]
#[command(name = "sound")]
#[command(about = "A CLI tool for generating and playing musical melodies")]
#[command(version = "0.1.0")]
struct Cli {
    /// Note positions in the scale (1-based), comma-separated
    #[arg(
        value_delimiter = ',',
        help = "Comma-separated note positions (e.g., 1,3,5,8). Defaults to full scale if not provided."
    )]
    notes: Vec<usize>,

    /// Scale to use for the melody
    #[arg(short, long, default_value = "major")]
    #[arg(help = "Scale: major, minor, dorian, blues, japanese, etc.")]
    scale: String,

    /// Key/root note for the melody
    #[arg(short, long, default_value = "C")]
    #[arg(help = "Root note: C, D, E, F, G, A, B (with optional # for sharps)")]
    key: String,
}

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
        "minor_pentatonic" | "minor_penta" => {
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

fn create_melody_config(cli: &Cli) -> Result<MelodyConfig, String> {
    println!(
        "CLI args: scale={}, key={}, notes={:?}",
        cli.scale, cli.key, cli.notes
    );

    // Parse scale
    let (scale_intervals, scale_name) = get_scale_by_name(&cli.scale)?;

    // Parse key/root note
    let note = parse_note_from_string(&cli.key)?;
    let key = Key::new(note, 4);

    // Use default notes if none provided (full scale)
    let note_positions = if cli.notes.is_empty() {
        vec![1, 2, 3, 4, 5, 6, 7, 8]
    } else {
        cli.notes.clone()
    };

    let config = MelodyConfig {
        scale_name,
        scale_intervals,
        note_positions,
        key,
    };

    Ok(config)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parse command line arguments
    let cli = Cli::parse();

    // Set up audio output
    let stream_handle =
        OutputStreamBuilder::open_default_stream().expect("Failed to open audio stream");
    let sink = Sink::connect_new(&stream_handle.mixer());

    let sample_rate = 44100;

    // Create melody configuration from CLI arguments
    match create_melody_config(&cli) {
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
        }
        Err(error) => {
            eprintln!("❌ Error: {}", error);
            eprintln!("");
            eprintln!("💡 Examples:");
            eprintln!("   cargo run -- 1,2,3,4,5,6,7,8 --scale major");
            eprintln!("   cargo run -- 1,3,5,8 --scale minor --key A");
            eprintln!("   cargo run -- 1,2,3,5,6 --scale dorian --key D");
            eprintln!("   cargo run -- 1,2,3,4,5 --scale japanese");
            eprintln!("");
            eprintln!("Run 'cargo run -- --help' for more information.");
            std::process::exit(1);
        }
    }

    Ok(())
}
