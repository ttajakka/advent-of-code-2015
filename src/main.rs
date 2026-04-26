use core::panic;

use advent_of_code_2015::{init, solve, util};
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

    Init {
        first_day: u8,
        last_day: u8,
        #[arg(long)]
        make_lib: bool,
    },
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
        Command::Init {
            first_day,
            last_day,
            make_lib,
        } => {
            if first_day < 1 {
                panic!("first_day must be at least 1");
            }
            if last_day > 25 {
                panic!("last_day must be at most 25");
            }
            if first_day > last_day {
                panic!("last_day must be greater than or equal to first_day")
            }
            init::init(first_day, last_day);

            if make_lib {
                init::make_lib(last_day);
            }
        }
    };
}
