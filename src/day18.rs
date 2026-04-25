use crate::util;

pub fn solve(level: &util::Level) -> String {
    match level {
        util::Level::One => part1(),
        util::Level::Two => part2(),
    }
}

pub fn part1() -> String {
    let input = util::load_input(18);

    let mut state: Vec<Vec<_>> = input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| match c {
                    '.' => 0,
                    '#' => 1,
                    _ => panic!(),
                })
                .collect()
        })
        .collect();

    for _ in 0..100 {
        state = update(state);
    }

    let answer = state
        .iter()
        .map(|r| r.iter().sum::<i32>())
        .sum::<i32>()
        .to_string();

    answer
}

fn update(state: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut out = state.clone();

    for i in 0..state.len() {
        for j in 0..state[i].len() {
            let c = count_neighbors(&state, i, j);
            if state[i][j] == 0 && c == 3 {
                out[i][j] = 1;
            } else if state[i][j] == 1 && !(2 == c || c == 3) {
                out[i][j] = 0;
            }
        }
    }

    out
}

fn count_neighbors(state: &Vec<Vec<i32>>, i: usize, j: usize) -> i32 {
    // corners
    if i == 0 && j == 0 {
        return state[i + 1][j] + state[i][j + 1] + state[i + 1][j + 1];
    }

    if i == 0 && j == state[i].len() - 1 {
        return state[i + 1][j] + state[i][j - 1] + state[i + 1][j - 1];
    }

    if i == state.len() - 1 && j == 0 {
        return state[i - 1][j] + state[i][j + 1] + state[i - 1][j + 1];
    }

    if i == state.len() - 1 && j == state[i].len() - 1 {
        return state[i - 1][j] + state[i][j - 1] + state[i - 1][j - 1];
    }

    // edges
    if i == 0 {
        return state[i][j - 1]
            + state[i][j + 1]
            + state[i + 1][j - 1]
            + state[i + 1][j]
            + state[i + 1][j + 1];
    }

    if i == state.len() - 1 {
        return state[i][j - 1]
            + state[i][j + 1]
            + state[i - 1][j - 1]
            + state[i - 1][j]
            + state[i - 1][j + 1];
    }

    if j == 0 {
        return state[i - 1][j]
            + state[i + 1][j]
            + state[i - 1][j + 1]
            + state[i][j + 1]
            + state[i + 1][j + 1];
    }

    if j == state.len() - 1 {
        return state[i - 1][j]
            + state[i + 1][j]
            + state[i - 1][j - 1]
            + state[i][j - 1]
            + state[i + 1][j - 1];
    }
    // interior
    return state[i - 1][j - 1]
        + state[i - 1][j]
        + state[i - 1][j + 1]
        + state[i][j - 1]
        + state[i][j + 1]
        + state[i + 1][j - 1]
        + state[i + 1][j]
        + state[i + 1][j + 1];
}

pub fn part2() -> String {
    let input = util::load_input(18);

    let mut state: Vec<Vec<_>> = input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| match c {
                    '.' => 0,
                    '#' => 1,
                    _ => panic!(),
                })
                .collect()
        })
        .collect();

    let height = state.len();
    let width = state[0].len();

    for _ in 0..100 {
        state = update(state);

        state[0][0] = 1;
        state[0][width - 1] = 1;
        state[height - 1][0] = 1;
        state[height - 1][width - 1] = 1;
    }

    let answer = state
        .iter()
        .map(|r| r.iter().sum::<i32>())
        .sum::<i32>()
        .to_string();

    answer
}
