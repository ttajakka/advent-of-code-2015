use crate::util;

pub fn solve(level: &util::Level) -> String {
    match level {
        util::Level::One => part1(),
        util::Level::Two => part2(),
    }
}

pub fn part1() -> String {
    let input = util::load_input(4);
    let input = input.strip_suffix("\n").unwrap();

    let mut answer = 0;
    loop {
        answer += 1;
        let digest = md5::compute(format!("{input}{answer}")).0;
        if digest[0] == 0 && digest[1] == 0 && (digest[2] >> 4) == 0 {
            break;
        }
    }

    let answer = answer.to_string();

    answer
}

pub fn part2() -> String {
    let input = util::load_input(4);

    let input = input.strip_suffix("\n").unwrap();

    let mut answer = 0;
    loop {
        answer += 1;
        let digest = md5::compute(format!("{input}{answer}")).0;
        if digest[0] == 0 && digest[1] == 0 && digest[2] == 0 {
            break;
        }
    }

    let answer = answer.to_string();
    
    answer
}
