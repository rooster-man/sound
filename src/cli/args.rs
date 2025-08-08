use clap::Parser;

/// Sound CLI - Generate and play melodies with customizable scales and keys
#[derive(Parser, Debug)]
#[command(name = "sound")]
#[command(about = "A CLI tool for generating and playing musical melodies")]
#[command(version = "0.1.0")]
#[derive(Clone)]
pub struct Args {
    /// Enhanced note notation with rests, sustains, and modal octave shifts
    #[arg(
        help = "Enhanced note notation: digits 1-9 for scale positions, dots (.) for rests, dashes (-) extend notes. Each symbol defaults to sixteenth note duration (use --duration to change). Octave shifts: ^ (up) and v (down) change register for all following notes. Examples: 12345 (notes 1-5), 1^234 (note 1 normal, then shift up, notes 2-4 higher)"
    )]
    pub notes: Vec<String>,

    /// Scale to use for the melody
    #[arg(short, long, default_value = "major")]
    #[arg(help = "Scale: major, minor, dorian, blues, japanese, etc.")]
    pub scale: String,

    /// Wave form to use
    #[arg(short, long, default_value = "sine")]
    #[arg(help = "sine, triangle, square, pulse, sawtooth")]
    pub wave: String,

    /// Key/root note for the melody
    #[arg(short, long, default_value = "C")]
    #[arg(help = "Root note: C, D, E, F, G, A, B (with optional # for sharps)")]
    pub key: String,

    /// Tempo in beats per minute (quarter note = 1 beat)
    #[arg(short, long, default_value = "120")]
    #[arg(help = "Tempo in BPM (beats per minute). Higher = faster, lower = slower")]
    pub bpm: u32,

    /// Loop the melody continuously
    #[arg(short, long)]
    #[arg(help = "Play the melody in a continuous loop. Press Ctrl+C to stop")]
    pub r#loop: bool,

    /// Base duration for each note symbol
    #[arg(short, long, default_value = "sixteenth")]
    #[arg(
        help = "Duration each note symbol represents: whole/1, half/2, quarter/4, eighth/8, sixteenth/16"
    )]
    pub duration: String,
}
