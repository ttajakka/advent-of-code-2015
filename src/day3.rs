use std::collections::HashSet;

use crate::util;

pub fn solve(level: &util::Level) -> String {
    match level {
        util::Level::One => part1(),
        util::Level::Two => part2(),
    }
}

pub fn part1() -> String {
    let input = util::load_input(3);

    let mut visited = HashSet::new();
    visited.insert((0,0));

    input.chars().map(|c| match c {
        'v' => (0, -1),
        '>' => (1, 0),
        '^' => (0, 1),
        '<' => (-1, 0),
        _ => panic!("unknown character"),
    }).fold((0,0), |position, motion| {
        let new_position = (position.0 + motion.0, position.1 + motion.1);
        visited.insert(new_position);
        new_position
    });

    let answer = visited.len().to_string();

    answer
}

pub fn part2() -> String {
    let input = util::load_input(3);

    let mut visited = HashSet::new();
    visited.insert((0,0));

    let chars = input.chars();

    chars.step_by(2).map(|c| match c {
        'v' => (0, -1),
        '>' => (1, 0),
        '^' => (0, 1),
        '<' => (-1, 0),
        _ => panic!("unknown character"),
    }).fold((0,0), |position, motion| {
        let new_position = (position.0 + motion.0, position.1 + motion.1);
        visited.insert(new_position);
        new_position
    });
    
    let mut chars = input.chars();
    chars.next();

    chars.step_by(2).map(|c| match c {
        'v' => (0, -1),
        '>' => (1, 0),
        '^' => (0, 1),
        '<' => (-1, 0),
        _ => panic!("unknown character"),
    }).fold((0,0), |position, motion| {
        let new_position = (position.0 + motion.0, position.1 + motion.1);
        visited.insert(new_position);
        new_position
    });

    let answer = visited.len().to_string();

    answer
}
