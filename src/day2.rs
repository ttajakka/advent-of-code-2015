use crate::util;

use regex::Regex;

pub fn solve(level: &util::Level) -> String {
    match level {
        util::Level::One => part1(),
        util::Level::Two => part2(),
    }
}

pub fn part1() -> String {
    let input = util::load_input(2);

    let parser = PresentParser::new();

    let answer = input
        .lines()
        .map(|l| parser.parse(&l))
        .fold(0, |sum, p| sum + p.wrapping_required());

    let answer = answer.to_string();

    answer
}

pub fn part2() -> String {
    let input = util::load_input(2);

    let parser = PresentParser::new();

    let answer = input
        .lines()
        .map(|l| parser.parse(&l))
        .fold(0, |sum, p| sum + p.bow_required());

    let answer = answer.to_string();

    answer
}

struct Present {
    length: u32,
    width: u32,
    height: u32,
}

impl Present {
    fn volume(&self) -> u32 {
        self.length * self.width * self.height
    }

    fn wrapping_required(&self) -> u32 {
        let lw = self.length * self.width;
        let lh = self.length * self.height;
        let wh = self.width * self.height;
        let smallest = smallest_of_three(lw, lh, wh);

        smallest + 2 * (lw + lh + wh)
    }

    fn bow_required(&self) -> u32 {
        let lw = self.length + self.width;
        let lh = self.length + self.height;
        let wh = self.width + self.height;
        let shortest_perimeter = 2 * smallest_of_three(lw, lh, wh);

        shortest_perimeter + self.volume()
    }
}

fn smallest_of_three(a: u32, b: u32, c: u32) -> u32 {
    if a <= b {
        if a <= c { a } else { c }
    } else {
        if b <= c { b } else { c }
    }
}

struct PresentParser {
    re: Regex,
}

impl PresentParser {
    fn new() -> Self {
        Self {
            re: Regex::new(r"([1-9][0-9]*)x([1-9][0-9]*)x([1-9][0-9]*)").unwrap(),
        }
    }

    fn parse(&self, hay: &str) -> Present {
        let Some(caps) = self.re.captures(hay) else {
            panic!("cannot parse present dimensions");
        };
        let (_, [l, w, h]) = caps.extract();

        Present {
            length: l.parse().unwrap(),
            width: w.parse().unwrap(),
            height: h.parse().unwrap(),
        }
    }
}
