use super::{args::Args, play::play, read::read};
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(flatten)]
    args: Args,

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
            read(&args);
        }
        None => {
            play(&cli.args);
        }
    }
}
