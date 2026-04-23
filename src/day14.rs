use crate::util;

pub fn solve(level: &util::Level) -> String {
    match level {
        util::Level::One => part1(),
        util::Level::Two => part2(),
    }
}

const TOTAL_TIME: u32 = 2503;

#[derive(Debug)]
struct ReindeerProps {
    speed: u32,
    movement_duration: u32,
    rest_duration: u32,
}

#[derive(Debug)]
enum MovementState {
    Moved(u32),
    Rested(u32),
}

#[derive(Debug)]
struct Reindeer {
    props: ReindeerProps,
    state: MovementState,
    travelled: u32,
    score: u32,
}

impl Reindeer {
    fn progress(&mut self) {
        match self.state {
            MovementState::Moved(t) => {
                if t == self.props.movement_duration {
                    self.state = MovementState::Rested(1);
                } else {
                    self.state = MovementState::Moved(t + 1);
                    self.travelled += self.props.speed;
                }
            }
            MovementState::Rested(t) => {
                if t == self.props.rest_duration {
                    self.state = MovementState::Moved(1);
                    self.travelled += self.props.speed;
                } else {
                    self.state = MovementState::Rested(t + 1);
                }
            }
        }
    }
}

impl ReindeerProps {
    fn calculate_travelled_distance(&self, total_time: u32) -> u32 {
        let cycle_length = self.movement_duration + self.rest_duration;
        let movement_rest_cycles = total_time / cycle_length;
        let remainder = (total_time % cycle_length).min(self.movement_duration);

        self.speed * (movement_rest_cycles * self.movement_duration + remainder)
    }
}

fn parse_input(input: String) -> Vec<ReindeerProps> {
    let mut props = vec![];

    for line in input.lines() {
        let parts: Vec<_> = line.split_whitespace().collect();
        props.push(ReindeerProps {
            speed: parts[3].parse().expect("speed should be 3rd"),
            movement_duration: parts[6].parse().expect("movement duration should be 6th"),
            rest_duration: parts[13].parse().expect("rest duration should be 13th"),
        })
    }

    props
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
    let input = util::load_input(14);

    let props = parse_input(input);

    let mut reindeers: Vec<_> = props
        .into_iter()
        .map(|props| Reindeer {
            props,
            state: MovementState::Moved(0),
            travelled: 0,
            score: 0,
        })
        .collect();

    for _ in 0..TOTAL_TIME {
        reindeers.iter_mut().for_each(|r| r.progress());
        let max_distance = reindeers.iter().map(|r| r.travelled).max().unwrap();
        reindeers.iter_mut().for_each(|r| {
            if r.travelled == max_distance {
                r.score += 1;
            }
        });
    }

    let answer = reindeers.iter().map(|r| r.score).max().unwrap();

    let answer = answer.to_string();

    answer
}
