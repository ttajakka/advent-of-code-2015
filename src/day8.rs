use crate::util;

pub fn solve(level: &util::Level) -> String {
    match level {
        util::Level::One => part1(),
        util::Level::Two => part2(),
    }
}

pub fn part1() -> String {
    let input = util::load_input(8);

    let mut answer = 0;

    for line in input.lines() {
        answer += line.len() as i64;
        answer -= count_escaped(line);
    }

    answer.to_string()
}

pub fn part2() -> String {
    let input = util::load_input(8);

    let mut answer = 0;

    for line in input.lines() {
        let ct = count_encoded(line);
        answer += ct;
        answer -= line.len() as i64;
    }

    answer.to_string()
}

fn count_escaped(line: &str) -> i64 {
    let mut count = 0;

    let chars: Vec<_> = line.chars().collect();

    let mut i = 1;
    loop {
        if i == chars.len() - 1 {
            break;
        }

        match chars[i] {
            '\\' => match chars[i + 1] {
                '\\' | '"' => {
                    i += 2;
                }
                'x' => {
                    i += 4;
                }
                _ => panic!("impossible"),
            },
            _ => i += 1,
        }

        count += 1;
    }

    count
}

fn count_encoded(line: &str) -> i64 {
    let mut count = 0;

    for c in line.chars() {
        match c {
            '\\' | '"' => count += 2,
            _ => count += 1,
        }
    }

    count + 2 // include starting and ending quotation marks
}
