use advent_of_code_2015::{solve, init, util};
use clap::{Parser, Subcommand};

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Solve {
        #[arg(long)]
        submit: bool,
        day: u8,
        level: u8,
    },

    Init,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Command::Solve { day, level, submit } => {
            let level = match level {
                1 => util::Level::One,
                2 => util::Level::Two,
                _ => panic!("level must be 1 or 2, but {level} was provided"),
            };
            solve(day, level, submit)
        }
        Command::Init => {
            init::init();
        }
    };
}
