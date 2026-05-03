use std::collections::HashMap;

use crate::util;

pub fn solve(level: &util::Level) -> String {
    match level {
        util::Level::One => part1(),
        util::Level::Two => part2(),
    }
}

pub fn part1() -> String {
    let input = util::load_input(20);

    let input: u64 = input.lines().next().unwrap().parse::<u64>().unwrap();

    let mut answer = 0;
    let mut largest = 0;

    for i in 2.. { // sum_of_factors doesn't work for input = 1
        let presents = sum_of_factors(i) * 10;
        if presents >= input {
            answer = i;
            break;
        } else if presents > largest {
            largest = presents;
        }
    }

    let answer = answer.to_string();

    answer
}

fn sum_of_factors(input: u64) -> u64 {
    let factors_with_multiplicities = factor_with_multiplicities(input);

    factors_with_multiplicities
        .iter()
        .map(|(p, e)| (p.pow(*e + 1) - 1) / (p - 1))
        .product()
}

fn factor_with_multiplicities(input: u64) -> Vec<(u64, u32)> {
    let mut factors: HashMap<u64, u32> = HashMap::new();
    let mut input = input;
    'outer: loop {
        let mut cand = 2;
        while cand * cand <= input {
            if input % cand == 0 {
                input = input / cand;
                let entry = factors.entry(cand).or_insert(0);
                *entry += 1;

                continue 'outer;
            }

            cand += 1;
        }

        break;
    }

    *factors.entry(input).or_insert(0) += 1;

    factors.into_iter().collect()
}

pub fn part2() -> String {
    let _input = util::load_input(20);

    let answer = String::new();

    answer
}
