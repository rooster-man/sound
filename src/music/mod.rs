pub mod duration;
pub mod interval;
pub mod key;
pub mod melody;
pub mod note;
pub mod util;

pub use melody::{Melody, MelodyConfig, NoteElement};
pub use util::{get_scale_by_name, parse_note_from_string, parse_note_notation};
