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

    // === BASIC SCALE PATTERNS ===
    pub const MAJOR_SCALE: [i32; 8] = [0, 2, 4, 5, 7, 9, 11, 12]; // Ionian mode
    pub const MINOR_SCALE: [i32; 8] = [0, 2, 3, 5, 7, 8, 10, 12]; // Natural minor (Aeolian mode)

    // === PENTATONIC SCALES ===
    pub const PENTATONIC_MAJOR: [i32; 6] = [0, 2, 4, 7, 9, 12];
    pub const PENTATONIC_MINOR: [i32; 6] = [0, 3, 5, 7, 10, 12];
    pub const PENTATONIC_BLUES: [i32; 6] = [0, 3, 5, 6, 7, 10]; // Minor pentatonic + blue note

    // === CHURCH MODES ===
    pub const DORIAN: [i32; 8] = [0, 2, 3, 5, 7, 9, 10, 12]; // Natural minor with raised 6th
    pub const PHRYGIAN: [i32; 8] = [0, 1, 3, 5, 7, 8, 10, 12]; // Natural minor with lowered 2nd
    pub const LYDIAN: [i32; 8] = [0, 2, 4, 6, 7, 9, 11, 12]; // Major with raised 4th
    pub const MIXOLYDIAN: [i32; 8] = [0, 2, 4, 5, 7, 9, 10, 12]; // Major with lowered 7th
    pub const LOCRIAN: [i32; 8] = [0, 1, 3, 5, 6, 8, 10, 12]; // Most dissonant mode

    // === MINOR SCALE VARIATIONS ===
    pub const HARMONIC_MINOR: [i32; 8] = [0, 2, 3, 5, 7, 8, 11, 12]; // Natural minor + raised 7th
    pub const MELODIC_MINOR: [i32; 8] = [0, 2, 3, 5, 7, 9, 11, 12]; // Natural minor + raised 6th & 7th

    // === BLUES SCALES ===
    pub const BLUES_MAJOR: [i32; 7] = [0, 2, 3, 4, 7, 9, 12]; // Major pentatonic + blue notes
    pub const BLUES_MINOR: [i32; 7] = [0, 3, 5, 6, 7, 10, 12]; // Minor pentatonic + blue note

    // === CHROMATIC AND WHOLE TONE ===
    pub const CHROMATIC: [i32; 13] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]; // All 12 notes
    pub const WHOLE_TONE: [i32; 7] = [0, 2, 4, 6, 8, 10, 12]; // Only whole steps

    // === WORLD MUSIC SCALES ===
    pub const JAPANESE_HIRAJOSHI: [i32; 6] = [0, 2, 3, 7, 8, 12]; // Traditional Japanese
    pub const JAPANESE_KUMOI: [i32; 6] = [0, 2, 3, 7, 9, 12]; // Traditional Japanese
    pub const ARABIC_MAQAM: [i32; 8] = [0, 1, 4, 5, 7, 8, 11, 12]; // Middle Eastern
    pub const SPANISH_GYPSY: [i32; 8] = [0, 1, 4, 5, 7, 8, 10, 12]; // Flamenco/Gypsy
    pub const JEWISH_AHAVA_RABA: [i32; 8] = [0, 1, 4, 5, 7, 8, 10, 12]; // Jewish/Klezmer

    // === JAZZ SCALES ===
    pub const BEBOP_MAJOR: [i32; 9] = [0, 2, 4, 5, 7, 8, 9, 11, 12]; // Major + chromatic passing tone
    pub const BEBOP_MINOR: [i32; 9] = [0, 2, 3, 5, 7, 8, 9, 10, 12]; // Minor + chromatic passing tone
    pub const DIMINISHED: [i32; 9] = [0, 2, 3, 5, 6, 8, 9, 11, 12]; // Symmetric diminished
    pub const ALTERED: [i32; 8] = [0, 1, 3, 4, 6, 8, 10, 12]; // Super Locrian (jazz)

    // === EXOTIC SCALES ===
    pub const HUNGARIAN_MINOR: [i32; 8] = [0, 2, 3, 6, 7, 8, 11, 12]; // Dramatic minor variation
    pub const NEAPOLITAN_MINOR: [i32; 8] = [0, 1, 3, 5, 7, 8, 11, 12]; // Classical exotic
    pub const PERSIAN: [i32; 8] = [0, 1, 4, 5, 6, 8, 11, 12]; // Persian/Iranian
    pub const ENIGMATIC: [i32; 8] = [0, 1, 4, 6, 8, 10, 11, 12]; // Very exotic sound

    // === SYNTHETIC SCALES ===
    pub const PROMETHEUS: [i32; 7] = [0, 2, 4, 6, 9, 10, 12]; // Scriabin's scale
    pub const TRITONE_SCALE: [i32; 7] = [0, 1, 4, 6, 7, 10, 12]; // Based on tritone intervals
    pub const DOUBLE_HARMONIC: [i32; 8] = [0, 1, 4, 5, 7, 8, 11, 12]; // Byzantine scale

    // Common chord patterns
    pub const MAJOR_TRIAD: [i32; 3] = [0, 4, 7];
    pub const MINOR_TRIAD: [i32; 3] = [0, 3, 7];
    pub const DIMINISHED_TRIAD: [i32; 3] = [0, 3, 6];
    pub const AUGMENTED_TRIAD: [i32; 3] = [0, 4, 8];
}
