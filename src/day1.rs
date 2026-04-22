use crate::util;

pub fn solve(level: &util::Level) -> String {
    match level {
        util::Level::One => part1(),
        util::Level::Two => part2(),
    }
}

pub fn part1() -> String {
    let input = util::load_input(1);

    let answer = input.chars().fold(0, |mut sum, c| {
        match c {
            '(' => sum += 1,
            ')' => sum -= 1,
            _ => panic!("illegal character"),
        }
        sum
    });

    answer.to_string()
}

pub fn part2() -> String {
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

    answer.to_string()
}
