use std::collections::HashMap;

use crate::util;

pub fn solve(level: &util::Level) -> String {
    match level {
        util::Level::One => part1(),
        util::Level::Two => part2(),
    }
}

const MFCSAM: &str = "children: 3
cats: 7
samoyeds: 2
pomeranians: 3
akitas: 0
vizslas: 0
goldfish: 5
trees: 3
cars: 2
perfumes: 1
";

fn parse_sues(input: String) -> Vec<HashMap<String, u32>> {
    input
        .lines()
        .map(|l| {
            let l = l.replace(":", "").replace(",", "");
            let parts: Vec<_> = l.split_whitespace().collect();

            assert_eq!(parts.len(), 8);

            let mut h = HashMap::new();
            h.insert(parts[2].to_string(), parts[3].parse().unwrap());
            h.insert(parts[4].to_string(), parts[5].parse().unwrap());
            h.insert(parts[6].to_string(), parts[7].parse().unwrap());

            h
        })
        .collect()
}

pub fn part1() -> String {
    let input = util::load_input(16);

    let sues = parse_sues(input);

    let mut mfcsam: HashMap<String, u32> = HashMap::new();
    let pairs: Vec<(String, u32)> = MFCSAM
        .lines()
        .map(|l| {
            let parts: Vec<_> = l.split(": ").collect();
            (parts[0].to_string(), parts[1].parse().unwrap())
        }).collect();

    for p in pairs {
        mfcsam.insert(p.0, p.1);
    }

    let mut answer = 0;

    'outer: for (i, sue) in sues.iter().enumerate() {
        for (k,v) in sue {
            if mfcsam[k] != *v {
                continue 'outer;
            }
        }

        answer = i+1; // sues are 1-indexed >:(
        break
    }

    let answer = answer.to_string();

    answer
}

pub fn part2() -> String {
    let _input = util::load_input(16);

    let answer = String::new();

    answer
}
