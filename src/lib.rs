pub mod init;
pub mod util;

pub mod day1;
pub mod day10;
pub mod day11;
pub mod day12;
pub mod day13;
pub mod day14;
pub mod day15;
pub mod day16;
pub mod day17;
pub mod day18;
pub mod day19;
pub mod day2;
pub mod day20;
pub mod day21;
pub mod day22;
pub mod day23;
pub mod day24;
pub mod day25;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day8;
pub mod day9;

pub fn solve(day: u8, level: util::Level, submit: bool) {
    let answer = match day {
        1 => day1::solve(&level),
        2 => day2::solve(&level),
        3 => day3::solve(&level),
        4 => day4::solve(&level),
        5 => day5::solve(&level),
        6 => day6::solve(&level),
        7 => day7::solve(&level),
        8 => day8::solve(&level),
        9 => day9::solve(&level),
        10 => day10::solve(&level),
        11 => day11::solve(&level),
        12 => day12::solve(&level),
        13 => day13::solve(&level),
        14 => day14::solve(&level),
        15 => day15::solve(&level),
        16 => day16::solve(&level),
        17 => day17::solve(&level),
        18 => day18::solve(&level),
        19 => day19::solve(&level),
        20 => day20::solve(&level),
        21 => day21::solve(&level),
        22 => day22::solve(&level),
        23 => day23::solve(&level),
        24 => day24::solve(&level),
        25 => day25::solve(&level),
        _ => todo!(),
    };

    println!("Solved day {day}, level {level} puzzle.");
    println!("Answer: {answer}");

    if submit {
        println!("submitting...");
        util::submit(day, level, answer);
    }
}
