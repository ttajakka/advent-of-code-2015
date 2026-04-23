use std::collections::HashMap;

use itertools::Itertools;

use crate::util;

pub fn solve(level: &util::Level) -> String {
    match level {
        util::Level::One => part1(),
        util::Level::Two => part2(),
    }
}

pub fn part1() -> String {
    let input = util::load_input(13);

    let graph = build_graph(input);

    let mut answer = 0;

    for perm in graph.keys().permutations(graph.len()) {
        let happiness = score(&graph, perm);
        if happiness > answer {
            answer = happiness;
        }
    }

    let answer = answer.to_string();

    answer
}

fn build_graph(input: String) -> HashMap<String, Vec<(String, i32)>> {
    let mut graph = HashMap::new();

    for line in input.lines() {
        let parts: Vec<_> = line.split(&[' ', '.']).collect();
        let source = parts[0].to_string();
        let entry = graph.entry(source).or_insert(vec![]);

        let dest = parts[10].to_string();
        let sign = match parts[2] {
            "gain" => 1,
            "lose" => -1,
            _ => panic!("malformed sign indicator: {line}"),
        };
        let amount: i32 = parts[3]
            .parse()
            .expect(&format!("malformed amount: {}", line));

        entry.push((dest, sign * amount));
    }

    graph
}

fn score(graph: &HashMap<String, Vec<(String, i32)>>, permutation: Vec<&String>) -> i32 {
    let length = permutation.len();

    let mut happiness = 0;
    for i in 0..length {
        let left_seat = permutation[(i + length - 1) % length];
        let right_seat = permutation[(i + 1) % length];

        let happiness_modifiers = graph.get(permutation[i]).expect("entry should be there");

        let left_score = happiness_modifiers
            .iter()
            .find(|x| x.0 == *left_seat)
            .expect("left entry must be there")
            .1;
        let right_score = happiness_modifiers
            .iter()
            .find(|x| x.0 == *right_seat)
            .expect("left entry must be there")
            .1;

        happiness = happiness + left_score + right_score;
    }

    happiness
}

pub fn part2() -> String {
    let input = util::load_input(13);

    let mut graph = build_graph(input);

    let mut my_scores = vec![];
    for k in graph.keys() {
        my_scores.push((k.clone(), 0));
    }

    for k in graph.values_mut() {
        k.push(("me".to_string(), 0));
    }
    let _ = graph.insert("me".to_string(), my_scores); // entry won't be there

    let mut answer = 0;

    for perm in graph.keys().permutations(graph.len()) {
        let happiness = score(&graph, perm);
        if happiness > answer {
            answer = happiness;
        }
    }

    let answer = answer.to_string();

    answer
}
