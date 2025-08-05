use super::args::Args;
use crate::audio::square_wave::SquareWave;
use crate::music::interval;
use crate::music::key::Key;
use crate::music::note::{MusicNote, Note};
use crossterm::event::{
    read, Event, KeyCode, KeyEventKind, KeyModifiers, KeyboardEnhancementFlags,
    PopKeyboardEnhancementFlags, PushKeyboardEnhancementFlags,
};

use crossterm::execute;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use rodio::{cpal::BufferSize, OutputStreamBuilder, Sink, Source};
use std::collections::HashMap;
use std::time::Duration;

pub fn jam(_args: &Args) -> Result<(), Box<dyn std::error::Error>> {
    let stream_handle = OutputStreamBuilder::from_default_device()?
        .with_buffer_size(BufferSize::Fixed(512))
        .open_stream()?;

    enable_raw_mode()?;

    let mut stdout = std::io::stdout();

    execute!(
        stdout,
        PushKeyboardEnhancementFlags(KeyboardEnhancementFlags::REPORT_EVENT_TYPES)
    )?;

    let mut active_keys: HashMap<(KeyCode, KeyModifiers), Sink> = HashMap::new();

    let key = Key::new(Note::C, 4);
    let minor_scale = interval::MINOR_SCALE;

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

                    if let KeyCode::Char(c) = key_event.code {
                        if c.is_ascii_digit() && c >= '1' && c <= '7' {
                            if !active_keys.contains_key(&key_id) {
                                let scale_index = c.to_digit(10).unwrap() as usize - 1;

                                let note = MusicNote::from_key_interval(
                                    &key,
                                    minor_scale[scale_index],
                                    Duration::from_secs(10),
                                );

                                let square_wave = SquareWave::from_note(&note, 44100);
                                let sink = Sink::connect_new(&stream_handle.mixer());

                                sink.append(square_wave.repeat_infinite());
                                active_keys.insert(key_id, sink);

                                println!("Playing note {}", c);
                            }
                        }
                    }
                }
                KeyEventKind::Release => {
                    if let Some(sink) = active_keys.remove(&key_id) {
                        sink.stop();
                        if let KeyCode::Char(c) = key_event.code {
                            println!("Stopped note {}", c);
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
