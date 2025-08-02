//! CLI-driven melody example
//! Run with: cargo run --example cli_melody "C|E|G|C"

use rodio::{OutputStreamBuilder, Sink};
use std::env;
use std::time::Duration;

use sound::duration::duration;
use sound::interval::interval;
use sound::{Key, Melody, Note};

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
        "REST" | "R" => Ok(Note::Rest),
        _ => Err(format!("Unknown note: {}", note_str)),
    }
}

fn create_melody_from_notes(note_strings: Vec<String>) -> Result<Melody, String> {
    let mut melody = Melody::new().set_key(Key::new(Note::C, 4));

    for note_str in note_strings {
        // let note = parse_note_from_string(&note_str)?;
        let interval = note_str.parse::<usize>().unwrap();
        let i = interval::MINOR_SCALE[interval - 1];
        melody = melody.add_interval(
            interval::MINOR_SCALE[interval - 1],
            duration::quarter_note(),
        );
    }

    Ok(melody)
}

fn process_args(args: Vec<String>) -> Result<Vec<String>, String> {
    if args.len() < 2 {
        return Err("Usage: program \"note1|note2|note3\" (e.g., \"C|E|G|C\")".to_string());
    }

    let parts: Vec<String> = args[1].split("|").map(|s| s.trim().to_string()).collect();

    if parts.is_empty() {
        return Err("No notes provided".to_string());
    }

    Ok(parts)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    // Set up audio
    let stream_handle =
        OutputStreamBuilder::open_default_stream().expect("Failed to open audio stream");
    let sink = Sink::connect_new(&stream_handle.mixer());

    match process_args(args) {
        Ok(note_strings) => {
            println!("🎵 Creating melody from notes: {:?}", note_strings);
            let note_count = note_strings.len();

            match create_melody_from_notes(note_strings) {
                Ok(melody) => {
                    println!("🎶 Playing your custom melody...");
                    melody.play(&sink, 44100);
                    std::thread::sleep(Duration::from_millis(note_count as u64 * 300));
                }
                Err(error) => {
                    println!("❌ Error creating melody: {}", error);
                    return Ok(());
                }
            }
        }
        Err(error) => {
            println!("❌ {}", error);
            println!("💡 Example: cargo run --example cli_melody \"C|E|G|C\"");
            return Ok(());
        }
    }

    println!("✨ Melody complete!");
    Ok(())
}
