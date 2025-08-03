use super::{args::Args, play::play};
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Play a melody
    Play(Args),
    /// Read a melody from a file
    Read(Args),
}

pub fn run_cli() {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Play(args)) => {
            play(&args);
        }
        Some(Commands::Read(args)) => {
            println!("Reading melody");
        }
        None => {
            println!("No command provided")
        }
    }
}
