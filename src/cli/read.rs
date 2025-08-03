use super::args::Args;
use super::play::create_melody_config;
use crate::music::melody::Melody;
use rodio::{OutputStreamBuilder, Sink};
use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

pub fn read(args: &Args) {
    println!("Reading melody from file: {:?}", args.notes);
    // Construct a dynamic controller and mixer, stream_handle, and sink.
    let stream_handle =
        OutputStreamBuilder::open_default_stream().expect("Failed to open audio stream");
    // let sink = Sink::connect_new(&stream_handle.mixer());

    let mut melodies = Vec::new();
    // File hosts.txt must exist in the current path
    if let Ok(lines) = read_lines(args.notes.first().unwrap()) {
        // Consumes the iterator, returns an (Optional) String
        for mut line in lines.map_while(Result::ok) {
            let mut a = args.clone();
            line.retain(|c| !c.is_whitespace());
            a.notes = vec![line];
            let config = create_melody_config(&a).unwrap();
            let melody = Melody::new(config);
            melodies.push(melody);
        }

        let mut sinks = Vec::new();
        for melody in &melodies {
            let sink = Sink::connect_new(&stream_handle.mixer()); // << new “channel”
                                                                  // sink.pause();
            melody.play(&sink);
            sinks.push(sink);
        }

        for sink in &sinks {
            sink.sleep_until_end();
        }
        // std::thread::sleep(Duration::from_millis(iteration_duration_ms));
    }
}

// The output is wrapped in a Result to allow matching on errors.
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
