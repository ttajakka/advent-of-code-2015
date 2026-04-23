use crate::util;

pub fn solve(level: &util::Level) -> String {
    match level {
        util::Level::One => part1(),
        util::Level::Two => part2(),
    }
}

fn parse_input(input: String) -> Vec<Vec<i32>> {
    input
        .replace(',', "")
        .lines()
        .map(|l| {
            let parts: Vec<_> = l.split_whitespace().collect();
            vec![2_usize, 4, 6, 8, 10]
                .into_iter()
                .map(|i| parts[i].parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect()
}
pub fn part1() -> String {
    let input = util::load_input(15);

    let columns = parse_input(input);
    let mut answer = 0;

    // loop over all subsets of size 4 in a set the set {1,...,100}
    for i in 0..103 {
        for j in i..103 {
            for k in j..103 {
                let weights = vec![i, j - i - 1, k - j - 1, 103 - k - 1];

                let score: i32 = (0_usize..4)
                    .map(|x| {
                        columns
                            .iter()
                            .enumerate()
                            .map(|(s, c)| weights[s] * c[x])
                            .sum::<i32>()
                            .max(0)
                    })
                    .product();

                if score > answer {
                    answer = score;
                }
            }
        }
    }

    let answer = answer.to_string();

    answer
}

pub fn part2() -> String {
    let input = util::load_input(15);

    let columns = parse_input(input);

    let mut answer = 0;

    // loop over all subsets of size 4 in a set the set {1,...,100}
    for i in 0..103 {
        for j in i..103 {
            for k in j..103 {
                let weights = vec![i, j - i - 1, k - j - 1, 103 - k - 1];

                let calories: i32 = columns
                    .iter()
                    .enumerate()
                    .map(|(s, c)| weights[s] * c[4])
                    .sum();
                let calories = calories.max(0);

                if calories != 500 {
                    continue;
                }

                let score: i32 = (0_usize..4)
                    .map(|x| {
                        columns
                            .iter()
                            .enumerate()
                            .map(|(s, c)| weights[s] * c[x])
                            .sum::<i32>()
                            .max(0)
                    })
                    .product();

                if score > answer {
                    answer = score;
                }
            }
        }
    }

    let answer = answer.to_string();

    answer
}
