//! Demo application for the sound library
use clap::Parser;
use rodio::{OutputStreamBuilder, Sink};
use std::time::Duration;

use sound::interval::interval;
use sound::{Key, Melody, Note};

/// Represents different musical elements in our enhanced notation
#[derive(Debug, Clone)]
enum NoteElement {
    /// A note at a specific scale position
    Note(usize),
    /// A sixteenth-note rest
    Rest,
    /// A sixteenth-note sustain (extends the previous note)
    Sustain,
}

/// Sound CLI - Generate and play melodies with customizable scales and keys
#[derive(Parser, Debug)]
#[command(name = "sound")]
#[command(about = "A CLI tool for generating and playing musical melodies")]
#[command(version = "0.1.0")]
struct Cli {
    /// Enhanced note notation with rests and sustains  
    #[arg(
        help = "Enhanced note notation: digits 1-9 for scale positions (sixteenth-note duration), dots (.) for rests, dashes (-) extend the previous note. Consecutive digits work: '123' = notes 1,2,3. Examples: '1..35' (note 1, two rests, note 3, note 5), '12-4' (note 1, note 2 extended, note 4)"
    )]
    notes: Vec<String>,

    /// Scale to use for the melody
    #[arg(short, long, default_value = "major")]
    #[arg(help = "Scale: major, minor, dorian, blues, japanese, etc.")]
    scale: String,

    /// Key/root note for the melody
    #[arg(short, long, default_value = "C")]
    #[arg(help = "Root note: C, D, E, F, G, A, B (with optional # for sharps)")]
    key: String,

    /// Tempo in beats per minute (quarter note = 1 beat)
    #[arg(short, long, default_value = "120")]
    #[arg(help = "Tempo in BPM (beats per minute). Higher = faster, lower = slower")]
    bpm: u32,
}

// Configuration struct for melody generation
#[derive(Debug)]
struct MelodyConfig {
    scale_name: String,
    scale_intervals: &'static [i32],
    note_elements: Vec<NoteElement>,
    key: Key,
    bpm: u32,
}

impl Default for MelodyConfig {
    fn default() -> Self {
        Self {
            scale_name: "major".to_string(),
            scale_intervals: &interval::MAJOR_SCALE,
            note_elements: vec![
                NoteElement::Note(1),
                NoteElement::Note(2),
                NoteElement::Note(3),
                NoteElement::Note(4),
                NoteElement::Note(5),
                NoteElement::Note(6),
                NoteElement::Note(7),
                NoteElement::Note(8),
            ], // Default C major scale
            key: Key::new(Note::C, 4),
            bpm: 120,
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

/// Parse enhanced note notation into a sequence of NoteElement
/// Examples: "1..3-5" -> [Note(1), Rest, Rest, Note(3), Sustain, Note(5)]
/// "123" -> [Note(1), Note(2), Note(3)] (consecutive digits treated as separate notes)
fn parse_note_notation(note_strings: &[String]) -> Result<Vec<NoteElement>, String> {
    let mut elements = Vec::new();

    for note_string in note_strings {
        let mut chars = note_string.chars().peekable();

        while let Some(ch) = chars.next() {
            match ch {
                '1'..='9' => {
                    // Each digit is treated as a separate note (1-9 only, no 0)
                    let position = ch.to_digit(10).unwrap() as usize;
                    elements.push(NoteElement::Note(position));
                }
                '0' => {
                    return Err("Note position 0 is invalid. Use positions 1-9.".to_string());
                }
                '.' => {
                    // Add a rest
                    elements.push(NoteElement::Rest);
                }
                '-' => {
                    // Add a sustain
                    elements.push(NoteElement::Sustain);
                }
                ' ' | '\t' => {
                    // Whitespace - ignore
                }
                _ => {
                    return Err(format!("Invalid character '{}' in note notation. Use only digits 1-9, dots (.), and dashes (-)", ch));
                }
            }
        }
    }

    if elements.is_empty() {
        return Err("No notes provided".to_string());
    }

    Ok(elements)
}

fn create_melody_config(cli: &Cli) -> Result<MelodyConfig, String> {
    println!(
        "CLI args: scale={}, key={}, notes={:?}, bpm={}",
        cli.scale, cli.key, cli.notes, cli.bpm
    );

    // Validate BPM range
    if cli.bpm == 0 || cli.bpm > 500 {
        return Err("BPM must be between 1 and 500".to_string());
    }

    // Parse scale
    let (scale_intervals, scale_name) = get_scale_by_name(&cli.scale)?;

    // Parse key/root note
    let note = parse_note_from_string(&cli.key)?;
    let key = Key::new(note, 4);

    // Parse note elements or use default
    let note_elements = if cli.notes.is_empty() {
        vec![
            NoteElement::Note(1),
            NoteElement::Note(2),
            NoteElement::Note(3),
            NoteElement::Note(4),
            NoteElement::Note(5),
            NoteElement::Note(6),
            NoteElement::Note(7),
            NoteElement::Note(8),
        ]
    } else {
        parse_note_notation(&cli.notes)?
    };

    let config = MelodyConfig {
        scale_name,
        scale_intervals,
        note_elements,
        key,
        bpm: cli.bpm,
    };

    Ok(config)
}

/// Calculate note durations based on BPM (quarter note = 1 beat)
fn calculate_durations(bpm: u32) -> (Duration, Duration) {
    let quarter_note_ms = 60_000 / bpm; // milliseconds per quarter note
    let sixteenth_note_ms = quarter_note_ms / 4; // sixteenth note is 1/4 of quarter note

    (
        Duration::from_millis(quarter_note_ms as u64), // quarter note duration
        Duration::from_millis(sixteenth_note_ms as u64), // sixteenth note duration
    )
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

            // Calculate durations based on BPM
            let (_quarter_note_duration, sixteenth_note_duration) = calculate_durations(config.bpm);

            // Create and play the custom melody
            println!("\n🎶 Playing your custom melody at {} BPM...", config.bpm);
            let mut melody = Melody::in_key(config.key);
            let mut i = 0;

            while i < config.note_elements.len() {
                match &config.note_elements[i] {
                    NoteElement::Note(position) => {
                        if *position == 0 || *position > config.scale_intervals.len() {
                            println!(
                                "⚠️  Warning: Note position {} is out of range for this scale",
                                position
                            );
                            i += 1;
                            continue;
                        }
                        let interval = config.scale_intervals[position - 1];

                        // Count sustains that follow this note
                        let mut sustain_count = 0;
                        let mut j = i + 1;
                        while j < config.note_elements.len() {
                            match &config.note_elements[j] {
                                NoteElement::Sustain => {
                                    sustain_count += 1;
                                    j += 1;
                                }
                                _ => break,
                            }
                        }

                        // Calculate duration: base sixteenth note + sustains (all BPM-based)
                        let base_duration = sixteenth_note_duration;
                        let sustain_duration = Duration::from_millis(
                            sixteenth_note_duration.as_millis() as u64 * sustain_count as u64,
                        );
                        let total_duration = base_duration + sustain_duration;

                        melody = melody.add_interval(interval, total_duration);

                        // Skip past the sustains we just processed
                        i = j;
                    }
                    NoteElement::Rest => {
                        melody = melody.add_rest(sixteenth_note_duration);
                        i += 1;
                    }
                    NoteElement::Sustain => {
                        // Sustains without a preceding note are treated as rests
                        melody = melody.add_rest(sixteenth_note_duration);
                        i += 1;
                    }
                }
            }

            melody.play(&sink, sample_rate);

            // Calculate sleep duration based on elements (BPM-accurate)
            let total_elements = config.note_elements.len();
            let duration_ms = total_elements as u64 * sixteenth_note_duration.as_millis() as u64;
            std::thread::sleep(Duration::from_millis(duration_ms));

            println!("✨ Custom melody complete!");
        }
        Err(error) => {
            eprintln!("❌ Error: {}", error);
            eprintln!("");
            eprintln!("💡 Examples:");
            eprintln!(
                "   cargo run -- 12345 --scale major --bpm 120     # Notes 1,2,3,4,5 at 120 BPM"
            );
            eprintln!(
                "   cargo run -- 1..35 --scale minor --key A --bpm 80  # Slow tempo (80 BPM)"
            );
            eprintln!(
                "   cargo run -- 12-4 --scale dorian --key D --bpm 160 # Fast tempo (160 BPM)"
            );
            eprintln!(
                "   cargo run -- 1.2.3.4.5 --scale japanese --bpm 100 # Notes with rests at 100 BPM"
            );
            eprintln!("");
            eprintln!("Run 'cargo run -- --help' for more information.");
            std::process::exit(1);
        }
    }

    Ok(())
}
