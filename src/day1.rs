use crate::util;

pub fn solve(level: &util::Level) -> i64 {
    match level {
        util::Level::One => part1(),
        util::Level::Two => part2(),
    }
}

pub fn part1() -> i64 {
    let input = util::load_input(1);

    let answer = input.chars().fold(0, |mut sum, c| {
        match c {
            '(' => sum += 1,
            ')' => sum -= 1,
            _ => panic!("illegal character"),
        }
        sum
    });

    answer
}

pub fn part2() -> i64 {
    let input = util::load_input(1);

    let mut answer = 0;
    let mut sum = 0;
    for c in input.chars() {
        answer += 1;
        match c {
            '(' => sum += 1,
            ')' => sum -= 1,
            _ => panic!("illegal character"),
        };
        if sum < 0 {break};
    }

    answer
}
