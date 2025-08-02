# 🎼 Musical Scales Reference

This document lists all available scales in the sound library, organized by category.

## Usage

```rust
use sound::{Key, Melody, Note};
use sound::duration::duration;
use sound::interval::interval;

// Play any scale in any key
let melody = Melody::in_key(Key::new(Note::C, 4))
    .add_intervals(&interval::DORIAN, duration::quarter_note());
```

## 🎵 Available Scales

### BASIC SCALE PATTERNS

- `MAJOR_SCALE` - Standard major scale (Ionian mode) - **Happy, bright**
- `MINOR_SCALE` - Natural minor scale (Aeolian mode) - **Sad, dark**

### PENTATONIC SCALES

- `PENTATONIC_MAJOR` - 5-note major scale - **Simple, folk-like**
- `PENTATONIC_MINOR` - 5-note minor scale - **Blues, rock**
- `PENTATONIC_BLUES` - Minor pentatonic + blue note - **Blues, rock**

### CHURCH MODES

- `DORIAN` - Minor with raised 6th - **Jazz, Celtic**
- `PHRYGIAN` - Minor with lowered 2nd - **Spanish, dark**
- `LYDIAN` - Major with raised 4th - **Dreamy, floating**
- `MIXOLYDIAN` - Major with lowered 7th - **Blues, rock**
- `LOCRIAN` - Most dissonant mode - **Experimental, tense**

### MINOR SCALE VARIATIONS

- `HARMONIC_MINOR` - Natural minor + raised 7th - **Classical, dramatic**
- `MELODIC_MINOR` - Natural minor + raised 6th & 7th - **Jazz, classical**

### BLUES SCALES

- `BLUES_MAJOR` - Major pentatonic + blue notes - **Happy blues**
- `BLUES_MINOR` - Minor pentatonic + blue note - **Classic blues**

### CHROMATIC AND WHOLE TONE

- `CHROMATIC` - All 12 notes - **Jazz, experimental**
- `WHOLE_TONE` - Only whole steps - **Dreamy, impressionist**

### WORLD MUSIC SCALES

- `JAPANESE_HIRAJOSHI` - Traditional Japanese - **Meditative, zen**
- `JAPANESE_KUMOI` - Traditional Japanese - **Peaceful, Asian**
- `ARABIC_MAQAM` - Middle Eastern - **Exotic, Middle Eastern**
- `SPANISH_GYPSY` - Flamenco/Gypsy - **Passionate, Spanish**
- `JEWISH_AHAVA_RABA` - Jewish/Klezmer - **Traditional Jewish**

### JAZZ SCALES

- `BEBOP_MAJOR` - Major + chromatic passing tone - **Jazz, bebop**
- `BEBOP_MINOR` - Minor + chromatic passing tone - **Jazz, bebop**
- `DIMINISHED` - Symmetric diminished - **Jazz, experimental**
- `ALTERED` - Super Locrian - **Jazz, advanced**

### EXOTIC SCALES

- `HUNGARIAN_MINOR` - Dramatic minor variation - **Gypsy, dramatic**
- `NEAPOLITAN_MINOR` - Classical exotic - **Classical, unusual**
- `PERSIAN` - Persian/Iranian - **Middle Eastern**
- `ENIGMATIC` - Very exotic sound - **Mysterious, otherworldly**

### SYNTHETIC SCALES

- `PROMETHEUS` - Scriabin's scale - **Modern classical**
- `TRITONE_SCALE` - Based on tritone intervals - **Dissonant, modern**
- `DOUBLE_HARMONIC` - Byzantine scale - **Eastern, exotic**

## 🎯 Scale Characteristics Quick Reference

### **Happy/Bright**: MAJOR_SCALE, LYDIAN, PENTATONIC_MAJOR

### **Sad/Dark**: MINOR_SCALE, PHRYGIAN, LOCRIAN

### **Jazzy**: DORIAN, MIXOLYDIAN, BEBOP_MAJOR, BEBOP_MINOR

### **Bluesy**: BLUES_MINOR, BLUES_MAJOR, PENTATONIC_BLUES

### **Exotic**: ARABIC_MAQAM, HUNGARIAN_MINOR, PERSIAN, ENIGMATIC

### **Dreamy**: LYDIAN, WHOLE_TONE, JAPANESE_HIRAJOSHI

### **Dramatic**: HARMONIC_MINOR, HUNGARIAN_MINOR, SPANISH_GYPSY

## 🎪 Examples

### Play a Spooky Halloween Melody

```rust
// Hungarian Minor for dramatic/spooky effect
let spooky = Melody::in_key(Key::new(Note::D, 4))
    .add_intervals(&interval::HUNGARIAN_MINOR, duration::quarter_note());
```

### Create Asian-Inspired Music

```rust
// Japanese Hirajoshi for zen/meditative feel
let zen = Melody::in_key(Key::new(Note::A, 4))
    .add_intervals(&interval::JAPANESE_HIRAJOSHI, duration::half_note());
```

### Jazz Improvisation

```rust
// Dorian mode for jazz solos
let jazz = Melody::in_key(Key::new(Note::G, 4))
    .add_intervals(&interval::DORIAN, duration::eighth_note());
```

### Middle Eastern Flavor

```rust
// Arabic Maqam for exotic sound
let middle_eastern = Melody::in_key(Key::new(Note::E, 4))
    .add_intervals(&interval::ARABIC_MAQAM, duration::quarter_note());
```

Run `cargo run --example scale_showcase` to hear all scales in action! 🎵
