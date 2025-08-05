use super::args::Args;
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

    println!("\n🎹 Multi-Octave Jam Mode 🎹");
    println!("Scale: {}", args.scale);
    println!("Base key: {:?} (octave {})", key.root, key.octave);
    let scale_notes = scale_intervals.len() - 1; // Exclude octave
    println!("\nKeyboard Layout ({} notes per row):", scale_notes);
    println!("  Numbers 1-{}:  Octave {} (base)", scale_notes, key.octave);
    println!("  QWERTYUI:      Octave {} (+1)", key.octave + 1);
    println!("  ASDFGHJK:      Octave {} (+2)", key.octave + 2);
    println!("  ZXCVBNM,:      Octave {} (+3)", key.octave + 3);
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
                        println!("Base octave: {}", key.octave);
                    }
                    if key_event.code == KeyCode::Down {
                        key.octave -= 1;
                        println!("Base octave: {}", key.octave);
                    }

                    if let KeyCode::Char(c) = key_event.code {
                        let (octave_offset, scale_index_opt) =
                            get_key_mapping(c, scale_intervals.len() - 1);

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

                                let settings = LimitSettings::default()
                                    .with_threshold(-6.0) // -6 dBFS threshold
                                    .with_attack(Duration::from_millis(5))
                                    .with_release(Duration::from_millis(100));

                                let limited = sine_wave.limit(settings);

                                sink.append(limited);
                                active_keys.insert(key_id, sink);

                                println!(
                                    "Playing note {} (octave {})",
                                    scale_index + 1,
                                    note_key.octave
                                );
                            }
                        }
                    }
                }
                KeyEventKind::Release => {
                    if let Some(sink) = active_keys.remove(&key_id) {
                        sink.stop();
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
