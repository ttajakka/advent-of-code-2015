use std::collections::{HashMap, HashSet};

use crate::util;

pub fn solve(level: &util::Level) -> String {
    match level {
        util::Level::One => part1(),
        util::Level::Two => part2(),
    }
}

pub fn part1() -> String {
    let input = util::load_input(19);

    let (replacements, original) = parse_input(input);

    let mut seen = HashSet::<String>::new();

    for (i, atom) in original.iter().enumerate() {
        if let Some(repls) = replacements.get(atom) {
            for r in repls {
                let (head, tail) = original.split_at(i + 1);
                let (head, _removed) = head.split_at(i);

                let mut head = head.to_owned();
                let mut r_owned = r.to_owned();
                let mut tail = tail.to_owned();

                head.append(&mut r_owned);
                head.append(&mut tail);

                seen.insert(head.join(""));
            }
        }
    }

    let answer = seen.len().to_string();

    answer
}

fn parse_input(input: String) -> (HashMap<String, Vec<Vec<String>>>, Vec<String>) {
    let mut replacements = HashMap::new();

    let mut parts = input.split("\n\n");
    let replacements_str = parts.next().unwrap();

    for l in replacements_str.lines() {
        let mut parts = l.split(" => ");
        let key = parts.next().unwrap().to_string();
        let values = split_molecule(parts.next().unwrap().to_string());

        replacements.entry(key).or_insert(vec![]).push(values);
    }

    let molecule = split_molecule(parts.next().unwrap().lines().next().unwrap().to_string());

    (replacements, molecule)
}

fn split_molecule(molecule_str: String) -> Vec<String> {
    let mut molecule_str = molecule_str;
    let mut molecule = vec![];

    while let Some(index) = molecule_str.rfind(char::is_uppercase) {
        let (new, atom) = molecule_str.split_at(index);
        molecule.push(atom.to_string());
        molecule_str = new.to_string();
    }

    molecule.reverse();

    molecule
}

pub fn part2() -> String {
    let _input = util::load_input(19);

    let answer = String::new();

    answer
}
