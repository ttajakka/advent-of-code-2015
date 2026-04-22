use crate::util;

pub fn solve(level: &util::Level) -> String {
    match level {
        util::Level::One => part1(),
        util::Level::Two => part2(),
    }
}

pub fn part1() -> String {
    let mut input = util::load_input(11).into_bytes();
    let len = input.len();
    _ = input.split_off(len - 1);
    // let len = input.len();
    // _ = input.split_off(len-1);

    // let mut answer = input;
    // loop {
    //     if is_valid(&answer) {
    //         break;
    //     }
    //     increment(&mut answer);
    // }

    let answer = find_first_valid_password(input);

    String::from_utf8(answer).unwrap()
}

fn find_first_valid_password(input: Vec<u8>) -> Vec<u8> {
    let mut answer = input;
    loop {
        if is_valid(&answer) {
            break;
        }
        increment(&mut answer);
    }

    answer.to_vec()
}

fn is_valid(input: &[u8]) -> bool {
    contains_three_consequtive(input) && no_i_o_l(input) && contains_two_pairs(input)
}

fn contains_three_consequtive(input: &[u8]) -> bool {
    for i in 2..input.len() {
        if input[i - 1] != input[i - 2] + 1 {
            continue;
        }

        if input[i] != input[i - 1] + 1 {
            continue;
        }

        return true;
    }
    false
}

fn no_i_o_l(input: &[u8]) -> bool {
    for c in input {
        let c = *c;
        if c == b'i' || c == b'o' || c == b'l' {
            return false;
        }
    }
    true
}

fn contains_two_pairs(input: &[u8]) -> bool {
    let mut has_first_pair = false;
    let mut first_char = b'a';

    for i in 1..input.len() {
        let c = input[i];
        if input[i - 1] != c {
            continue;
        }

        if has_first_pair && c != first_char {
            return true;
        }

        if !has_first_pair {
            first_char = c;
            has_first_pair = true;
        }
    }

    false
}

fn increment(input: &mut [u8]) {
    let mut carry = 1; // this is the increment
    let mut sum;
    for i in (0..input.len()).rev() {
        (sum, carry) = add_with_carry(input[i], carry);
        input[i] = sum;
    }
}

fn add_with_carry(current: u8, carry: u8) -> (u8, u8) {
    let mut sum = current + carry;
    let mut carry = 0;
    if sum > b'z' {
        sum = b'a';
        carry = 1;
    }

    (sum, carry)
}

pub fn part2() -> String {
    let input = util::load_input(11);
    let mut input = input.into_bytes();
    let len = input.len();
    _ = input.split_off(len - 1);

    let mut part_1_answer = find_first_valid_password(input);
    increment(&mut part_1_answer);

    let answer = String::from_utf8(find_first_valid_password(part_1_answer)).unwrap();

    answer
}
