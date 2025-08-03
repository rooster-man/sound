use super::args::Args;
use crate::{
    get_scale_by_name, music::melody::Melody, parse_note_from_string, parse_note_notation, Key,
    MelodyConfig, NoteElement,
};
use rodio::{OutputStreamBuilder, Sink};
use std::time::Duration;

pub fn play(args: &Args) {
    let stream_handle =
        OutputStreamBuilder::open_default_stream().expect("Failed to open audio stream");
    let sink = Sink::connect_new(&stream_handle.mixer());

    // Create melody configuration from CLI arguments
    match create_melody_config(&args) {
        Ok(config) => {
            println!("✅ Successfully parsed melody configuration:");
            println!("  🎼 Scale: {}", config.scale_name);
            println!("  🎹 Key: {:?}", config.key.root);
            println!("  ⏱️  Duration: {} notes", config.base_duration);

            let melody = Melody::new(config);

            // Calculate sleep duration for one iteration
            let total_elements = melody.note_elements.len();
            let iteration_duration_ms =
                total_elements as u64 * melody.base_note_duration.as_millis() as u64;

            // Play the melody (looping if requested)
            if melody.should_loop {
                println!(
                    "\n🔄 Playing your custom melody at {} BPM (looping - press Ctrl+C to stop)...",
                    melody.bpm
                );
                loop {
                    melody.play(&sink);
                    std::thread::sleep(Duration::from_millis(iteration_duration_ms));
                }
            } else {
                println!("\n🎶 Playing your custom melody at {} BPM...", melody.bpm);
                melody.play(&sink);
                std::thread::sleep(Duration::from_millis(iteration_duration_ms));
            }

            println!("✨ Custom melody complete!");
        }
        Err(error) => {
            eprintln!("❌ Error: {}", error);
            eprintln!("");
            eprintln!("💡 Examples:");
            eprintln!(
            "   cargo run -- 12345 --scale major --bpm 120     # Notes 1,2,3,4,5 at 120 BPM (sixteenth notes)"
        );
            eprintln!(
                "   cargo run -- 1^234 --scale minor --key A --duration 4  # Quarter note duration"
            );
            eprintln!(
            "   cargo run -- 1v23^45 --scale dorian --key D --duration 8  # Eighth note duration"
        );
            eprintln!(
                "   cargo run -- 12345 --duration 1 --bpm 60 --loop  # Very slow with whole notes"
            );
            eprintln!("");
            eprintln!("Run 'cargo run -- --help' for more information.");
            std::process::exit(1);
        }
    }
}

fn create_melody_config(args: &Args) -> Result<MelodyConfig, String> {
    println!(
        "CLI args: scale={}, key={}, notes={:?}, bpm={}, loop={}, duration={}",
        args.scale, args.key, args.notes, args.bpm, args.r#loop, args.duration
    );

    // Validate BPM range
    if args.bpm == 0 || args.bpm > 500 {
        return Err("BPM must be between 1 and 500".to_string());
    }

    // Validate duration
    match args.duration.to_lowercase().as_str() {
        "whole" | "1" | "half" | "2" | "quarter" | "4" | "eighth" | "8" | "sixteenth" | "16" => {}
        _ => {
            return Err(
                "Duration must be one of: whole/1, half/2, quarter/4, eighth/8, sixteenth/16"
                    .to_string(),
            )
        }
    }

    // Parse scale
    let (scale_intervals, scale_name) = get_scale_by_name(&args.scale)?;

    // Parse key/root note
    let note = parse_note_from_string(&args.key)?;
    let key = Key::new(note, 4);

    // Parse note elements or use default
    let note_elements = if args.notes.is_empty() {
        vec![
            NoteElement::Note(1, 0),
            NoteElement::Note(2, 0),
            NoteElement::Note(3, 0),
            NoteElement::Note(4, 0),
            NoteElement::Note(5, 0),
            NoteElement::Note(6, 0),
            NoteElement::Note(7, 0),
            NoteElement::Note(8, 0),
        ]
    } else {
        parse_note_notation(&args.notes)?
    };

    let config = MelodyConfig {
        scale_name,
        scale_intervals,
        note_elements,
        sample_rate: 44100,
        key,
        bpm: args.bpm,
        should_loop: args.r#loop,
        base_duration: args.duration.clone(),
    };

    Ok(config)
}
