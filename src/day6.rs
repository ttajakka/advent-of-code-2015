use regex::Regex;

use crate::util;

pub fn solve(level: &util::Level) -> i64 {
    match level {
        util::Level::One => part1(),
        util::Level::Two => part2(),
    }
}

struct Rectangle {
    left: usize,
    right: usize,
    top: usize,
    bottom: usize,
}

enum OpType {
    TurnOn,
    TurnOff,
    Toggle,
}
struct Operation {
    op_type: OpType,
    rect: Rectangle,
}

impl Operation {
    fn parse(input: &str) -> Self {
        let op_type;
        if Regex::new(r"turn on").unwrap().is_match(input) {
            op_type = OpType::TurnOn
        } else if Regex::new(r"turn of").unwrap().is_match(input) {
            op_type = OpType::TurnOff
        } else {
            op_type = OpType::Toggle
        };

        let rect_re = Regex::new(r"(\d+),(\d+) through (\d+),(\d+)").unwrap();
        let (_, [left, top, right, bottom]) = rect_re.captures(input).unwrap().extract();

        Self {
            op_type,
            rect: Rectangle {
                left: left.parse().unwrap(),
                right: right.parse().unwrap(),
                top: top.parse().unwrap(),
                bottom: bottom.parse().unwrap(),
            },
        }
    }
}

pub fn part1() -> i64 {
    let input = util::load_input(6);

    let mut grid = vec![vec![false; 1000]; 1000];

    for line in input.lines() {
        let Operation { op_type, rect } = Operation::parse(line);
        for i in rect.left..=rect.right {
            for j in rect.top..=rect.bottom {
                grid[i][j] = match op_type {
                    OpType::TurnOn => true,
                    OpType::TurnOff => false,
                    OpType::Toggle => !grid[i][j],
                }
            }
        }
    }

    let answer = grid.iter().fold(0, |sum, row| {
        sum + row
            .iter()
            .map(|a| match a {
                true => 1,
                false => 0,
            })
            .sum::<i64>()
    });

    answer
}

pub fn part2() -> i64 {
    let input = util::load_input(6);

    let mut grid = vec![vec![0_i64; 1000]; 1000];

    for line in input.lines() {
        let Operation { op_type, rect } = Operation::parse(line);
        for i in rect.left..=rect.right {
            for j in rect.top..=rect.bottom {
                grid[i][j] += match op_type {
                    OpType::TurnOn => 1,
                    OpType::TurnOff => {match grid[i][j] > 0 {
                        true => -1,
                        false => 0,
                    }},
                    OpType::Toggle => 2,
                }
            }
        }
    }

    let answer = grid.iter().map(|row| row.iter().sum::<i64>()).sum();

    answer
}
