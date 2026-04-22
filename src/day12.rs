use crate::util;
use serde_json::{self, Value};

pub fn solve(level: &util::Level) -> String {
    match level {
        util::Level::One => part1(),
        util::Level::Two => part2(),
    }
}

pub fn part1() -> String {
    let input = util::load_input(12);

    let answer = add_by_splitting(input);

    answer.to_string()
}

fn add_by_splitting(input: String) -> i64 {
    let mut answer = 0;
    for chunk in input.split(&[':', ',', '"', '{', '}', '[', ']']) {
        if let Ok(num) = chunk.parse::<i64>() {
            answer += num
        } else if chunk.contains(&['0', '1', '2', '3', '4', '5', '6', '7', '8', '9']) {
            println!("{chunk}")
        }
    }

    answer
}

pub fn part2() -> String {
    let input = util::load_input(12);

    let mut v: Value = serde_json::from_str(&input).unwrap();

    remove_red_objects(&mut v);

    let cleaned_input = v.to_string();

    let answer = add_by_splitting(cleaned_input).to_string();

    answer
}

fn remove_red_objects(v: &mut Value) {
    match v {
        Value::Null => return,
        Value::Bool(_) => return,
        Value::Number(_) => return,
        Value::String(_) => return,
        Value::Array(values) => {
            for val in values {
                remove_red_objects(val);
            }
        }
        Value::Object(map) => {
            let mut must_drop = false;
            for val in map.values() {
                if let Value::String(s) = val
                    && s == "red"
                {
                    must_drop = true;
                    break;
                }
            }

            if must_drop {
                *v = Value::Null
            } else {
                for (_, val) in map.iter_mut() {
                    remove_red_objects(val);
                }
            }
        }
    }
}
