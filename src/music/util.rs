use super::{interval, melody::NoteElement, note::Note};

pub fn get_scale_by_name(name: &str) -> Result<(&'static [i32], String), String> {
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

pub fn parse_note_from_string(note_str: &str) -> Result<Note, String> {
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

/// Parse enhanced note notation into a sequence of NoteElement with modal octave shifting
/// Examples: "1..3-5" -> [Note(1,0), Rest, Rest, Note(3,0), Sustain, Note(5,0)]
/// "123" -> [Note(1,0), Note(2,0), Note(3,0)] (consecutive digits treated as separate notes)
/// "1^234v5" -> [Note(1,0), Note(2,1), Note(3,1), Note(4,1), Note(5,0)] (modal octave shifting)
pub fn parse_note_notation(note_strings: &[String]) -> Result<Vec<NoteElement>, String> {
    let mut elements = Vec::new();

    for note_string in note_strings {
        let mut chars = note_string.chars().peekable();
        let mut current_octave_offset = 0i32; // Track current octave register

        while let Some(ch) = chars.next() {
            match ch {
                '1'..='9' => {
                    // Each digit is treated as a separate note (1-9 only, no 0)
                    let position = ch.to_digit(10).unwrap() as usize;
                    elements.push(NoteElement::Note(position, current_octave_offset));
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
                '^' => {
                    // Shift octave register up by one
                    current_octave_offset += 1;
                }
                'v' => {
                    // Shift octave register down by one
                    current_octave_offset -= 1;
                }
                ' ' | '\t' => {
                    // Whitespace - ignore
                }
                _ => {
                    return Err(format!("Invalid character '{}' in note notation. Use digits 1-9, dots (.), dashes (-), carets (^), and v's for octaves", ch));
                }
            }
        }
    }

    if elements.is_empty() {
        return Err("No notes provided".to_string());
    }

    Ok(elements)
}
