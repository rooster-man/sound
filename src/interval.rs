//! Musical interval constants and patterns

/// Common musical intervals (in semitones from root)
pub mod interval {
    pub const ROOT: i32 = 0; // Root note (unison)
    pub const MINOR_SECOND: i32 = 1;
    pub const MAJOR_SECOND: i32 = 2;
    pub const MINOR_THIRD: i32 = 3;
    pub const MAJOR_THIRD: i32 = 4;
    pub const PERFECT_FOURTH: i32 = 5;
    pub const TRITONE: i32 = 6; // Diminished fifth / Augmented fourth
    pub const PERFECT_FIFTH: i32 = 7;
    pub const MINOR_SIXTH: i32 = 8;
    pub const MAJOR_SIXTH: i32 = 9;
    pub const MINOR_SEVENTH: i32 = 10;
    pub const MAJOR_SEVENTH: i32 = 11;
    pub const OCTAVE: i32 = 12; // One octave up

    // Common scale patterns
    pub const MAJOR_SCALE: [i32; 8] = [0, 2, 4, 5, 7, 9, 11, 12];
    pub const MINOR_SCALE: [i32; 8] = [0, 2, 3, 5, 7, 8, 10, 12];
    pub const PENTATONIC_MAJOR: [i32; 6] = [0, 2, 4, 7, 9, 12];
    pub const PENTATONIC_MINOR: [i32; 6] = [0, 3, 5, 7, 10, 12];

    // Common chord patterns
    pub const MAJOR_TRIAD: [i32; 3] = [0, 4, 7];
    pub const MINOR_TRIAD: [i32; 3] = [0, 3, 7];
    pub const DIMINISHED_TRIAD: [i32; 3] = [0, 3, 6];
    pub const AUGMENTED_TRIAD: [i32; 3] = [0, 4, 8];
}