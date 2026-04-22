use std::collections::HashMap;

use regex::Regex;

use crate::util;

pub fn solve(level: &util::Level) -> String {
    match level {
        util::Level::One => part1(),
        util::Level::Two => part2(),
    }
}

enum GateType {
    And(String, String),
    OneAnd(String),
    Or(String, String),
    Not(String),
    RShift(String, u8),
    LShift(String, u8),
    Direct(String),
    Source(u16),
}

struct Gate {
    name: String,
    gate: GateType,
}

const SOURCE_PATTERN: &str = r"^\d+$";
const DIRECT: &str = r"^([a-z]+)$";
const AND_PATTERN: &str = r"(.+) AND (.+)";
const ONE_AND_PATTERN: &str = r"1 AND (.+)";
const OR_PATTERN: &str = r"(.+) OR (.+)";
const LSHIFT_PATTERN: &str = r"(.+) LSHIFT (.+)";
const RSHIFT_PATTERN: &str = r"(.+) RSHIFT (.+)";
const NOT_PATTERN: &str = r"NOT (.+)";

/// returns first input wire if any, second input wire if any, gate, and output wire
impl GateType {
    fn parse(input: &str) -> GateType {
        if Regex::new(SOURCE_PATTERN).unwrap().is_match(&input) {
            return GateType::Source(input.parse().unwrap());
        } else if Regex::new(DIRECT).unwrap().is_match(&input) {
            let (_, [first]) = Regex::new(DIRECT)
                .unwrap()
                .captures(&input)
                .unwrap()
                .extract();
            return GateType::Direct(first.to_string());
        } else if Regex::new(ONE_AND_PATTERN).unwrap().is_match(&input) {
            let (_, [first]) = Regex::new(ONE_AND_PATTERN)
                .unwrap()
                .captures(&input)
                .unwrap()
                .extract();
            return GateType::OneAnd(first.to_string());
        } else if Regex::new(AND_PATTERN).unwrap().is_match(&input) {
            let (_, [first, second]) = Regex::new(AND_PATTERN)
                .unwrap()
                .captures(&input)
                .unwrap()
                .extract();
            return GateType::And(first.to_string(), second.to_string());
        } else if Regex::new(OR_PATTERN).unwrap().is_match(&input) {
            let (_, [first, second]) = Regex::new(OR_PATTERN)
                .unwrap()
                .captures(&input)
                .unwrap()
                .extract();
            return GateType::Or(first.to_string(), second.to_string());
        } else if Regex::new(LSHIFT_PATTERN).unwrap().is_match(&input) {
            let (_, [first, second]) = Regex::new(LSHIFT_PATTERN)
                .unwrap()
                .captures(&input)
                .unwrap()
                .extract();
            return GateType::LShift(first.to_string(), second.parse().unwrap());
        } else if Regex::new(RSHIFT_PATTERN).unwrap().is_match(&input) {
            let (_, [first, second]) = Regex::new(RSHIFT_PATTERN)
                .unwrap()
                .captures(&input)
                .unwrap()
                .extract();
            return GateType::RShift(first.to_string(), second.parse().unwrap());
        } else if Regex::new(NOT_PATTERN).unwrap().is_match(&input) {
            let (_, [first]) = Regex::new(NOT_PATTERN)
                .unwrap()
                .captures(&input)
                .unwrap()
                .extract();
            return GateType::Not(first.to_string());
        } else {
            panic!("unparseable gate")
        };
    }
}

fn resolve_signal(
    name: &str,
    values: &mut HashMap<String, Option<u16>>,
    gates: &HashMap<String, Gate>,
) -> u16 {
    if let Some(value) = values.get(name).unwrap() {
        return *value;
    } else {
        let gate = gates.get(name).unwrap();
        let value = match &gate.gate {
            GateType::And(left, right) => {
                let left_value = resolve_signal(&gates.get(left).unwrap().name, values, gates);
                let right_value = resolve_signal(&gates.get(right).unwrap().name, values, gates);

                left_value & right_value
            }
            GateType::OneAnd(name) => {
                let value = resolve_signal(&gates.get(name).unwrap().name, values, gates);

                1 & value
            }
            GateType::Or(left, right) => {
                let left_value = resolve_signal(&gates.get(left).unwrap().name, values, gates);
                let right_value = resolve_signal(&gates.get(right).unwrap().name, values, gates);

                left_value | right_value
            }
            GateType::Not(name) => !resolve_signal(&gates.get(name).unwrap().name, values, gates),
            GateType::RShift(name, shift) => {
                resolve_signal(&gates.get(name).unwrap().name, values, gates) >> shift
            }
            GateType::LShift(name, shift) => {
                resolve_signal(&gates.get(name).unwrap().name, values, gates) << shift
            }
            GateType::Direct(name) => resolve_signal(&gates.get(name).unwrap().name, values, gates),
            GateType::Source(value) => *value,
        };

        *values.get_mut(name).unwrap() = Some(value);
        value
    }
}

pub fn part1() -> String {
    let (mut wire_values, output_gates) = prepare_network();

    let answer = resolve_signal(&"a", &mut wire_values, &output_gates);

    answer.to_string()
}

pub fn part2() -> String {
    let (mut wire_values, output_gates) = prepare_network();

    let answer_1 = resolve_signal(&"a", &mut wire_values, &output_gates);

    let (mut wire_values, mut output_gates) = prepare_network();

    *output_gates.get_mut("b").unwrap() = Gate{ name: "b".to_string(), gate: GateType::Source(answer_1) };

    let answer = resolve_signal(&"a", &mut wire_values, &output_gates);

    answer.to_string()
}

fn prepare_network() -> (HashMap<String, Option<u16>>, HashMap<String, Gate>) {
    let input = util::load_input(7);
    let mut wire_values: HashMap<String, Option<u16>> = HashMap::new();
    let mut output_gates: HashMap<String, Gate> = HashMap::new();

    for line in input.lines() {
        let (_, [input, output]) = Regex::new(r"(.+) -> (.+)")
            .unwrap()
            .captures(&line)
            .unwrap()
            .extract();
        wire_values.insert(output.to_string(), None);

        let name = output.to_string();
        output_gates.insert(
            name.clone(),
            Gate {
                name: name,
                gate: GateType::parse(&input),
            },
        );
    }

    (wire_values, output_gates)
}
