use crate::util;

pub fn solve(level: &util::Level) -> String {
    match level {
        util::Level::One => part1(),
        util::Level::Two => part2(),
    }
}

pub fn part1() -> String {
    let input = util::load_input(17);

    let containers: Vec<u32> = input.lines().map(|l| l.parse().unwrap()).collect();

    let len = containers.len() as u32;

    let mut answer = 0;

    for subset in 0..=2u32.pow(len) {
        // consider the bits of an integer between 0 and 2**len as encoding a subset of {1, ..., len}
        // 1. convert that subset into indices into the Vec `containers`
        let choices = (0..len).map(|i| subset >> i & 1);

        let count = choices
            .zip(containers.iter())
            .map(|(a, b)| a * b)
            .sum::<u32>();

        if count == 150 {
            answer += 1;
        }
    }

    let answer = answer.to_string();

    answer
}

pub fn part2() -> String {
    let input = util::load_input(17);

    let containers: Vec<u32> = input.lines().map(|l| l.parse().unwrap()).collect();

    let len = containers.len() as u32;

    let mut smallest = u32::MAX;
    let mut answer = 0;

    for subset in 0..=2u32.pow(len) {
        let choices = (0..len).map(|i| subset >> i & 1);

        // count is the number of containers in the subset
        let (count, sum) = choices
            .zip(containers.iter())
            .map(|(a, b)| (a, a * b))
            .fold((0, 0), |(a_c, a_s), (c, s)| (a_c + c, a_s + s));

        if sum == 150 {
            if count < smallest {
                smallest = count;
                answer = 1;
            } else if count == smallest {
                answer += 1;
            }
        }
    }

    let answer = answer.to_string();

    answer
}
