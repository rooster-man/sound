use super::args::Args;
use crate::audio::pulse::Pulse;
use crate::audio::square::Square;
use crate::audio::triangle::Triangle;
use crate::music::key::Key;
use crate::music::note::{MusicNote, Note};
use crate::music::util::get_scale_by_name;
use crossterm::event::{
    read, Event, KeyCode, KeyEventKind, KeyModifiers, KeyboardEnhancementFlags,
    PopKeyboardEnhancementFlags, PushKeyboardEnhancementFlags,
};

use crossterm::execute;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use rodio::{
    cpal::{traits::*, BufferSize, SupportedBufferSize},
    source::{LimitSettings, SineWave},
    OutputStream, OutputStreamBuilder, Sink, Source,
};
use std::collections::HashMap;
use std::time::Duration;

pub fn jam(args: &Args) -> Result<(), Box<dyn std::error::Error>> {
    let mut key = Key::new(Note::C, 4);
    let (scale_intervals, _scale_name) = get_scale_by_name(&args.scale)?;

    let stream_handle = build_stream_handle()?;

    println!("\n🎹 Chord-Scale Jazz Mode 🎹");
    println!("Melody Scale: {}", args.scale);
    println!("Base key: {:?} (octave {})", key.root, key.octave);
    let scale_notes = (scale_intervals.len() - 1).min(7); // Cap at 7 notes, exclude octave
    println!("\nMelody Notes ({} notes per row):", scale_notes);
    println!("  Numbers 1-{}:  Octave {} (base)", scale_notes, key.octave);
    println!("  QWERTYUI:      Octave {} (+1)", key.octave + 1);
    println!("  ASDFGHJK:      Octave {} (+2)", key.octave + 2);
    println!("  ZXCVBNM,:      Octave {} (+3)", key.octave + 3);
    println!("\nChords (-=[]\\;'):");
    print_chord_progression(&key, &scale_intervals, &args.scale);
    println!("\nTip: Play chords with right hand, improvise melodies with left hand!");
    println!(
        "The {} scale works great over these chord progressions.",
        args.scale
    );
    println!("\nControls:");
    println!("  ↑/↓ arrows: Change base octave");
    println!("  Ctrl+C: Exit");
    println!("\nPress and hold keys to play notes...\n");

    enable_raw_mode()?;
    let mut stdout = std::io::stdout();
    execute!(
        stdout,
        PushKeyboardEnhancementFlags(KeyboardEnhancementFlags::REPORT_EVENT_TYPES)
    )?;

    let mut active_keys: HashMap<(KeyCode, KeyModifiers), Sink> = HashMap::new();
    loop {
        if let Event::Key(key_event) = read()? {
            let key_id = (key_event.code, key_event.modifiers);

            match key_event.kind {
                KeyEventKind::Press => {
                    if key_event.code == KeyCode::Char('c')
                        && key_event.modifiers.contains(KeyModifiers::CONTROL)
                    {
                        break;
                    }

                    if key_event.code == KeyCode::Up {
                        key.octave += 1;
                    }
                    if key_event.code == KeyCode::Down {
                        key.octave -= 1;
                    }

                    if let KeyCode::Char(c) = key_event.code {
                        let (octave_offset, scale_index_opt) =
                            get_key_mapping(c, (scale_intervals.len() - 1).min(7));

                        if let Some(scale_index) = scale_index_opt {
                            if !active_keys.contains_key(&key_id) {
                                let note_key = Key::new(key.root, key.octave + octave_offset);
                                let note = MusicNote::from_key_interval(
                                    &note_key,
                                    scale_intervals[scale_index],
                                    Duration::from_secs(10),
                                );

                                let sink = Sink::connect_new(&stream_handle.mixer());
                                let sine_wave = SineWave::new(note.frequency());
                                let square_wave = Square::infinite(note.frequency(), 41000);
                                let pulse_wave =
                                    Pulse::new(note.frequency(), 41000, Duration::from_secs(10));
                                let triangle_wave =
                                    Triangle::new(note.frequency(), 41000, Duration::from_secs(10));

                                let settings = LimitSettings::default()
                                    .with_threshold(-6.0) // -6 dBFS threshold
                                    .with_attack(Duration::from_millis(5))
                                    .with_release(Duration::from_millis(100));

                                let limited = square_wave.limit(settings);

                                sink.append(limited);
                                active_keys.insert(key_id, sink);
                            }
                        } else {
                            // Check for chord mapping - use relative major chords for minor scales
                            let chord_info_opt =
                                get_chord_mapping(c, &key, &scale_intervals, &args.scale);

                            if let Some((chord_intervals, chord_name)) = chord_info_opt {
                                if !active_keys.contains_key(&key_id) {
                                    // Get the chord root key (relative major for minor scales)
                                    let chord_root_key =
                                        get_chord_root_key(&key, &scale_intervals, &args.scale);

                                    // Play the chord by creating multiple sinks
                                    let chord_sinks = play_chord(
                                        &stream_handle,
                                        &chord_root_key,
                                        &chord_intervals,
                                    );

                                    // Store all the sinks for this chord under the same key
                                    for (i, sink) in chord_sinks.into_iter().enumerate() {
                                        let chord_key = (
                                            key_event.code,
                                            KeyModifiers::from_bits_truncate(i as u8),
                                        );
                                        active_keys.insert(chord_key, sink);
                                    }

                                    println!("Playing {} chord", chord_name);
                                }
                            }
                        }
                    }
                }
                KeyEventKind::Release => {
                    // Remove the primary key
                    if let Some(sink) = active_keys.remove(&key_id) {
                        sink.stop();
                    }

                    // Also remove any chord keys (which use modified versions of the key_id)
                    let keys_to_remove: Vec<_> = active_keys
                        .keys()
                        .filter(|(code, _)| *code == key_event.code)
                        .cloned()
                        .collect();

                    for chord_key in keys_to_remove {
                        if let Some(sink) = active_keys.remove(&chord_key) {
                            sink.stop();
                        }
                    }
                }
                _ => {}
            }
        }
    }

    for (_, sink) in active_keys {
        sink.stop();
    }

    execute!(stdout, PopKeyboardEnhancementFlags)?;
    disable_raw_mode()?;
    println!("\nGoodbye!");

    Ok(())
}

fn build_stream_handle() -> Result<OutputStream, Box<dyn std::error::Error>> {
    let host = rodio::cpal::default_host();
    let device = host
        .default_output_device()
        .ok_or("No default output device found")?;

    println!("Audio Device: {}", device.name()?);

    let mut stream_handle = OutputStreamBuilder::from_default_device()?;
    // Get supported configurations
    let mut supported_configs = device.supported_output_configs()?;
    if let Some(config) = supported_configs.next() {
        println!(
            "Sample Rate Range: {} - {} Hz",
            config.min_sample_rate().0,
            config.max_sample_rate().0
        );
        println!("Channels: {}", config.channels());

        match config.buffer_size() {
            SupportedBufferSize::Range { min, max } => {
                println!("Supported Buffer Size Range: {} - {} frames", min, max);
                stream_handle = stream_handle.with_buffer_size(BufferSize::Fixed(*min));
            }
            SupportedBufferSize::Unknown => {
                println!("Buffer Size: Unknown");
            }
        }
    }

    let stream_handle = stream_handle.open_stream()?;

    println!(
        "Using Buffer Size: {:?}",
        &stream_handle.config().buffer_size()
    );

    Ok(stream_handle)
}

fn get_key_mapping(c: char, scale_length: usize) -> (u8, Option<usize>) {
    // Define the keyboard layout for each row
    let number_keys = ['1', '2', '3', '4', '5', '6', '7', '8', '9', '0'];
    let qwerty_keys = ['q', 'w', 'e', 'r', 't', 'y', 'u', 'i', 'o', 'p'];
    let asdf_keys = ['a', 's', 'd', 'f', 'g', 'h', 'j', 'k', 'l', ';'];
    let zxcv_keys = ['z', 'x', 'c', 'v', 'b', 'n', 'm', ',', '.', '/'];

    let c_lower = c.to_ascii_lowercase();

    // Check each row and return (octave_offset, scale_index) if within scale_length
    if let Some(pos) = number_keys.iter().position(|&k| k == c_lower) {
        if pos < scale_length {
            return (0, Some(pos));
        }
    }

    if let Some(pos) = qwerty_keys.iter().position(|&k| k == c_lower) {
        if pos < scale_length {
            return (1, Some(pos));
        }
    }

    if let Some(pos) = asdf_keys.iter().position(|&k| k == c_lower) {
        if pos < scale_length {
            return (2, Some(pos));
        }
    }

    if let Some(pos) = zxcv_keys.iter().position(|&k| k == c_lower) {
        if pos < scale_length {
            return (3, Some(pos));
        }
    }

    (0, None)
}

fn get_chord_mapping(
    c: char,
    _base_key: &Key,
    _scale_intervals: &[i32],
    _scale_name: &str,
) -> Option<(Vec<i32>, String)> {
    use crate::music::interval;

    let chord_keys = ['-', '=', '[', ']', ';', '\''];

    if let Some(pos) = chord_keys.iter().position(|&k| k == c) {
        if pos < 6 {
            // Always use major scale chord patterns for harmonic consistency
            return Some(build_diatonic_triad(&interval::MAJOR_SCALE, pos));
        }
    }

    None
}

fn get_chord_root_key(base_key: &Key, _scale_intervals: &[i32], scale_name: &str) -> Key {
    // For minor scales, find the relative major (3 semitones up)
    if is_minor_scale(scale_name) {
        let relative_major_root = base_key.note_at_interval(3).0;
        Key::new(relative_major_root, base_key.octave)
    } else {
        // For major and other scales, use the original key
        *base_key
    }
}

fn is_minor_scale(scale_name: &str) -> bool {
    match scale_name.to_lowercase().as_str() {
        "minor" | "natural_minor" | "harmonic_minor" | "melodic_minor" | "harmonic" | "dorian"
        | "phrygian" | "locrian" | "minor_pentatonic" | "minor_penta" | "blues" => true,
        _ => false,
    }
}

fn build_diatonic_triad(scale_intervals: &[i32], degree: usize) -> (Vec<i32>, String) {
    let scale_len = scale_intervals.len() - 1; // Exclude octave

    // Build triad using scale degrees (1st, 3rd, 5th of the scale)
    let root = scale_intervals[degree];
    let third = scale_intervals[(degree + 2) % scale_len];
    let fifth = scale_intervals[(degree + 4) % scale_len];

    // Normalize intervals to handle octave wrapping
    let mut third_interval = (third - root) % 12;
    let mut fifth_interval = (fifth - root) % 12;

    // Handle negative intervals
    if third_interval < 0 {
        third_interval += 12;
    }
    if fifth_interval < 0 {
        fifth_interval += 12;
    }

    let (chord_intervals, chord_name) = match (third_interval, fifth_interval) {
        (3, 7) => (vec![root, third, fifth], format!("{}m", degree + 1)), // Minor
        (4, 7) => (vec![root, third, fifth], format!("{}", degree + 1)),  // Major
        (3, 6) => (vec![root, third, fifth], format!("{}°", degree + 1)), // Diminished
        (4, 8) => (vec![root, third, fifth], format!("{}+", degree + 1)), // Augmented
        _ => (vec![root, third, fifth], format!("{}?", degree + 1)),      // Unknown
    };

    (chord_intervals, chord_name)
}

fn print_chord_progression(base_key: &Key, scale_intervals: &[i32], scale_name: &str) {
    use crate::music::interval;

    let chord_keys = ['-', '=', '[', ']', ';', '\''];
    let max_chords = 6; // Always show 6 chords from major scale

    // For minor scales, show relative major chords
    let chord_root_key = get_chord_root_key(base_key, scale_intervals, scale_name);
    let chord_context = if is_minor_scale(scale_name) {
        format!("from {} Major", format!("{:?}", chord_root_key.root))
    } else {
        format!("from {} Major", format!("{:?}", base_key.root))
    };

    print!("  ");
    for i in 0..max_chords {
        let (_, chord_name) = build_diatonic_triad(&interval::MAJOR_SCALE, i);
        print!("{} = {}   ", chord_keys[i], chord_name);
    }
    println!(" ({})", chord_context);

    // Show common chord progressions
    match scale_name.to_lowercase().as_str() {
        "major" => println!("  Common progressions: I-V-vi-IV, ii-V-I, I-vi-ii-V"),
        "minor" => println!("  Common progressions: VI-III-VII-IV (relative major), i-VI-III-VII"),
        "dorian" => println!("  Common progressions: VII-III-VI-II (relative major), i-IV-v-i"),
        _ => println!("  Try different chord combinations!"),
    }
}

fn play_chord(stream_handle: &rodio::OutputStream, base_key: &Key, intervals: &[i32]) -> Vec<Sink> {
    let mut sinks = Vec::new();

    for &interval in intervals {
        let note = MusicNote::from_key_interval(base_key, interval, Duration::from_secs(10));

        let sink = Sink::connect_new(&stream_handle.mixer());
        let sine_wave = Pulse::new(note.frequency(), 41000, Duration::from_secs(10));

        let settings = LimitSettings::default()
            .with_threshold(-6.0)
            .with_attack(Duration::from_millis(5))
            .with_release(Duration::from_millis(100));

        let limited = sine_wave.limit(settings);
        sink.append(limited);
        sinks.push(sink);
    }

    sinks
}
