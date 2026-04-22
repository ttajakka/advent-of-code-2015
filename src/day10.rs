use crate::util;

pub fn solve(level: &util::Level) -> String {
    match level {
        util::Level::One => part1(),
        util::Level::Two => part2(),
    }
}

pub fn part1() -> String {
    let input = util::load_input(10);

    let mut input = input
        .lines()
        .next()
        .unwrap()
        .chars()
        .map(|c| c.to_string().parse::<u32>().unwrap())
        .collect::<Vec<_>>();

    for _ in 0..40 {
        input = look_and_say(input);
    }

    let answer = input.len() as i64;

    answer.to_string()
}

pub fn part2() -> String {
    let input = util::load_input(10);

    let mut input = input
        .lines()
        .next()
        .unwrap()
        .chars()
        .map(|c| c.to_string().parse::<u32>().unwrap())
        .collect::<Vec<_>>();

    for _ in 0..50 {
        input = look_and_say(input);
    }

    let answer = input.len() as i64;

    answer.to_string()
}

fn look_and_say(input: Vec<u32>) -> Vec<u32> {
    let length = input.len();
    let mut out = Vec::new();

    let mut i = 0;
    while i < length {
        let mut count = 1;
        while i + count < length && input[i + count] == input[i] {
            count += 1;
        }
        out.push(count as u32);
        out.push(input[i]);

        i = i + count;
    }

    out
}
