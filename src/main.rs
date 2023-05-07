use std::error::Error;

use clap::{Parser, Subcommand};
use homework::{
    maze::{read_from_file, solve},
    numbers::convert_numbers,
};

#[derive(Parser)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Clone)]
enum Commands {
    /// Find path in the given maze with minimal number of turns
    Maze { path: String },
    /// Convert binary to decimal
    Numbers { path: String },
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
        Commands::Numbers { path } => {
            for n in convert_numbers(homework::numbers::read_from_file(&path)?) {
                match n {
                    Ok(n) => println!("{n}"),
                    Err(e) => println!("{e}"),
                }
            }
        }
    }

    Ok(())
}
