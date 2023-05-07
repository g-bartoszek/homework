use std::error::Error;

use clap::{Parser, Subcommand};
use homework::maze::{read_from_file, solve};

#[derive(Parser)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Clone)]
enum Commands {
    Maze { path: String },
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    match args.command {
        Commands::Maze { path } => {
            if let Some(solution) = solve(read_from_file(&path)?) {
                println!("{solution}")
            } else {
                println!("No solution")
            }
        }
    }

    Ok(())
}
