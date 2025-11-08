use crate::util;

pub fn part1(submit: bool) -> i64 {
    let input = util::load_input(1);

    let answer = input.chars().fold(0, |mut sum, c| {
        match c {
            '(' => sum += 1,
            ')' => sum -= 1,
            _ => panic!("illegal character"),
        }
        sum
    });

    println!("result: {answer}");

    if submit {
        util::submit(1, util::Level::One, answer);
    }

    answer
}

pub fn part2(submit: bool) -> i64 {
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

        println!("result: {answer}");

    if submit {
        util::submit(1, util::Level::Two, answer);
    }

    answer
}
