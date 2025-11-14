use crate::util;

pub fn solve(level: &util::Level) -> i64 {
    match level {
        util::Level::One => part1(),
        util::Level::Two => part2(),
    }
}

pub fn part1() -> i64 {
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

    answer
}

pub fn part2() -> i64 {
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

    answer
}
