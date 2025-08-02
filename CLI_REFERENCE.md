# 🎹 CLI Reference for Sound Library

## Basic Usage

```bash
cargo run "scale:SCALE_NAME|key:NOTE|notes:POSITIONS"
```

## Parameters

### 🎼 **Scales** (`scale:NAME`)

**Basic Scales:**

- `major` - Happy, bright sound
- `minor` - Sad, dark sound

**Pentatonic:**

- `pentatonic` or `penta` - Simple, folk-like (major)
- `pentatonic_minor` or `penta_minor` - Simple minor
- `blues` - Classic blues sound

**Church Modes:**

- `dorian` - Jazzy minor
- `phrygian` - Spanish, dark
- `lydian` - Dreamy, floating
- `mixolydian` - Bluesy major
- `locrian` - Dissonant, experimental

**Exotic Scales:**

- `harmonic_minor` or `harmonic` - Classical, dramatic
- `hungarian` - Very dramatic, gypsy
- `japanese` - Meditative, zen
- `arabic` - Middle Eastern
- `spanish` - Flamenco, passionate
- `whole_tone` or `wholetone` - Dreamy, impressionist
- `chromatic` - All 12 notes

### 🎹 **Keys** (`key:NOTE`)

- `C`, `D`, `E`, `F`, `G`, `A`, `B` (natural notes)
- `CS`/`C#`, `DS`/`D#`, `FS`/`F#`, `GS`/`G#`, `AS`/`A#` (sharp notes)

Default: `C`

### 🎵 **Notes** (`notes:POSITIONS`)

Scale positions as numbers (1-based):

- `1` = Root note of the scale
- `2` = Second note of the scale
- `3` = Third note of the scale
- etc.

Example: `notes:1,2,3,4,5,6,7,8` plays the full scale

## Examples

### Basic Scales

```bash
# C major scale
cargo run "scale:major|notes:1,2,3,4,5,6,7,8"

# A minor scale
cargo run "scale:minor|key:A|notes:1,2,3,4,5,6,7,8"

# Simple chord progression (1-3-5 pattern)
cargo run "scale:major|notes:1,3,5,8"
```

### Different Moods

```bash
# Jazz feel
cargo run "scale:dorian|key:D|notes:1,2,3,5,6,7,8"

# Blues lick
cargo run "scale:blues|key:E|notes:1,3,4,5,7"

# Spanish/Flamenco
cargo run "scale:spanish|key:E|notes:1,2,3,4,5"

# Japanese zen
cargo run "scale:japanese|key:A|notes:1,2,3,4,5"
```

### Creative Patterns

```bash
# Ascending and descending
cargo run "scale:lydian|notes:1,2,3,4,5,6,7,8,7,6,5,4,3,2,1"

# Skip patterns
cargo run "scale:major|notes:1,3,2,4,3,5,4,6,5,7,6,8"

# Dramatic Hungarian
cargo run "scale:hungarian|key:D|notes:1,2,3,4,5,6,7,8"
```

### World Music

```bash
# Middle Eastern
cargo run "scale:arabic|key:E|notes:1,2,3,4,5,6,7,8"

# Japanese traditional
cargo run "scale:japanese|notes:1,2,3,4,5,6"

# Spanish gypsy
cargo run "scale:spanish|key:A|notes:1,2,3,4,5,6,7,8"
```

### Experimental

```bash
# Whole tone (dreamy)
cargo run "scale:whole_tone|notes:1,2,3,4,5,6,7"

# Chromatic (all notes)
cargo run "scale:chromatic|notes:1,2,3,4,5,6,7,8,9,10,11,12,13"

# Dissonant Locrian
cargo run "scale:locrian|notes:1,2,3,4,5,6,7,8"
```

## Tips

1. **Scale Positions**: Numbers are 1-based (1 = first note of scale)
2. **Key Changes**: Same scale in different keys sounds higher/lower
3. **Pattern Reuse**: Same note pattern works in any scale/key
4. **Error Handling**: Invalid scales/keys show helpful error messages
5. **Default Demo**: Run without arguments to hear scale examples

## Error Examples

```bash
# Unknown scale
cargo run "scale:unknown|notes:1,2,3"
# → Shows error and available scales

# Invalid key
cargo run "scale:major|key:X|notes:1,2,3"
# → Shows error and valid keys

# No arguments
cargo run
# → Runs default demo with examples
```
