use std::collections::HashSet;

use crate::util;
use regex::Regex;

pub fn solve(level: &util::Level) -> String {
    match level {
        util::Level::One => part1(),
        util::Level::Two => part2(),
    }
}

pub fn part1() -> String {
    let input = util::load_input(5);

    let mut answer = 0;

    for line in input.lines() {
        if is_nice_part1(line) {
            answer += 1;
        }
    }

    let answer = answer.to_string();
    answer
}

pub fn part2() -> String {
    let input = util::load_input(5);

    let mut answer = 0;

    for line in input.lines() {
        if is_nice_part2(line) {
            answer += 1;
        }
    }

    let answer = answer.to_string();

    answer
}

fn is_nice_part1(line: &str) -> bool {
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

    return false;
}

fn is_nice_part2(line: &str) -> bool {
    let mut repeating_pair = false;
    let mut repeating_char = false;
    let mut pairs = HashSet::new();

    let mut prev = '0';
    let mut prevprev = '0';
    let mut prevprevprev = '0';
    for (i, c) in line.chars().enumerate() {
        if prevprev == c {
            repeating_char = true;
        }

        if i > 2 {
            pairs.insert((prevprevprev, prevprev));
        }
        if pairs.contains(&(prev, c)) {
            repeating_pair = true;
        }

        if repeating_char && repeating_pair {
            return true;
        }

        prevprevprev = prevprev;
        prevprev = prev;
        prev = c;
    }

    false
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn qjhvhtzxzqqjkmpb_is_nice() {
        assert_eq!(is_nice_part2("qjhvhtzxzqqjkmpb"), true);
    }

    #[test]
    fn xxyxx_is_nice() {
        assert_eq!(is_nice_part2("xxyxx"), true);
    }

    #[test]
    fn uurcxstgmygtbstg_is_naught() {
        assert_eq!(is_nice_part2("uurcxstgmygtbstg"), false);
    }

    #[test]
    fn ieodomkazucvgmuy_is_naught() {
        assert_eq!(is_nice_part2("ieodomkazucvgmuy"), false);
    }

    #[test]
    fn abab_is_nice() {
        assert_eq!(is_nice_part2("abab"), true);
    }
}
