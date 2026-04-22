use std::{collections::{HashMap, HashSet}, i64};

use itertools::Itertools;
use regex::Regex;

use crate::util;

pub fn solve(level: &util::Level) -> String {
    match level {
        util::Level::One => part1(),
        util::Level::Two => part2(),
    }
}

pub fn part1() -> String {
    let (locations, distances) = prepare_distances();

    let mut answer= i64::MAX;

    for p in locations.iter().permutations(locations.len()) {
        let mut length = 0;
        for i in 0..p.len()-1 {
            length += distances.get(&(p[i].to_string(), p[i+1].to_string())).unwrap()
        }

        if length < answer {
            answer = length
        }
    }

    answer.to_string()
}

pub fn part2() -> String {
    let (locations, distances) = prepare_distances();

    let mut answer= i64::MIN;

    for p in locations.iter().permutations(locations.len()) {
        let mut length = 0;
        for i in 0..p.len()-1 {
            length += distances.get(&(p[i].to_string(), p[i+1].to_string())).unwrap()
        }

        if length > answer {
            answer = length
        }
    }

    answer.to_string()
}

fn prepare_distances() -> (Vec<String>, HashMap<(String, String), i64>) {
let input = util::load_input(9);

    let mut locations = HashSet::new();
    let mut distances: HashMap<(String, String), i64> = HashMap::new();

    let pattern = Regex::new(r"(.+) to (.+) = (\d+)").unwrap();

    for line in input.lines() {
        let (_, [from, to, dist]) = pattern.captures(line).unwrap().extract();
        locations.insert(from.to_string());
        locations.insert(to.to_string());
        distances.insert((from.to_string(), to.to_string()), dist.parse().unwrap());
        distances.insert((to.to_string(), from.to_string()), dist.parse().unwrap());
    }

    (locations.into_iter().collect(), distances)
}