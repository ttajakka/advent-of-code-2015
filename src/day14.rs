use crate::util;

pub fn solve(level: &util::Level) -> String {
    match level {
        util::Level::One => part1(),
        util::Level::Two => part2(),
    }
}

const TOTAL_TIME: u32 = 2503;

struct Reindeer {
    speed: u32,
    movement_duration: u32,
    rest_duration: u32,
}

impl Reindeer {
    fn calculate_travelled_distance(&self, total_time: u32) -> u32 {
        let cycle_length = self.movement_duration + self.rest_duration;
        let movement_rest_cycles = total_time / cycle_length;
        let remainder = (total_time % cycle_length).min(self.movement_duration);

        self.speed * (movement_rest_cycles * self.movement_duration + remainder)
    }
}

fn parse_input(input: String) -> Vec<Reindeer> {
    let mut reindeers = vec![];

    for line in input.lines() {
        let parts: Vec<_> = line.split_whitespace().collect();
        reindeers.push(Reindeer {
            speed: parts[3].parse().expect("speed should be 3rd"),
            movement_duration: parts[6].parse().expect("movement duration should be 6th"),
            rest_duration: parts[13].parse().expect("rest duration should be 13th"),
        })
    }

    reindeers
}

pub fn part1() -> String {
    let input = util::load_input(14);

    let reindeers = parse_input(input);

    let mut answer = 0;
    for r in reindeers {
        let distance = r.calculate_travelled_distance(TOTAL_TIME);
        if distance > answer {
            answer = distance
        }
    }

    let answer = answer.to_string();

    answer
}

pub fn part2() -> String {
    let _input = util::load_input(14);

    let answer = String::new();

    answer
}
