use crate::util;

pub fn solve(level: &util::Level) -> String {
    match level {
        util::Level::One => part1(),
        util::Level::Two => part2(),
    }
}

pub fn part1() -> String {
    let input = util::load_input(12);

    let mut answer = 0;
    for chunk in input.split(&[':', ',', '"', '{', '}', '[', ']']) {
            if let Ok(num) = chunk.parse::<i64>() {
                answer += num
            } else if chunk.contains(&['0', '1', '2', '3', '4', '5', '6', '7', '8', '9']) {
                println!("{chunk}")
            }
    }


    answer.to_string()
}

pub fn part2() -> String {
    let _input = util::load_input(12);

    let answer = String::new();

    answer
}
