use crate::util;
use regex::Regex;

pub fn solve(level: &util::Level) -> i64 {
    match level {
        util::Level::One => part1(),
        util::Level::Two => part2(),
    }
}

pub fn part1() -> i64 {
    let input = util::load_input(5);

    let mut answer = 0;

    for line in input.lines() {
        if is_nice(line) {
            println!("{line}");
            answer += 1;
        }
    }

    answer
}

pub fn part2() -> i64 {
    let _input = util::load_input(5);

    let answer = 0;

    answer
}

fn is_nice(line: &str) -> bool {
    let disallowed = Regex::new(r"ab|cd|pq|xy").unwrap();
    if disallowed.is_match(line) {
        return false;
    }

    let vowels = Regex::new(r"((a|e|i|o|u).*){3,}").unwrap();
    if !vowels.is_match(line) {
        return false;
    }

    let mut chars = line.chars();
    let mut previous = chars.next().unwrap();
    for c in chars {
        if c == previous {
            return true;
        }
        previous = c;
    }

    return false
}