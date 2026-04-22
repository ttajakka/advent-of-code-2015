use std::{
    env,
    fs::{self, File, OpenOptions},
    io::{BufReader, Read, Write},
    path::Path,
};
use ureq;

const INPUT_DIR: &str = "input/";
const AOC_2015_BASE_URL: &str = "https://adventofcode.com/2015";

pub fn download_all_inputs() {
    for day in 1..=25 {
        download_input(day);
    }
}

pub enum Level {
    One,
    Two,
}

impl std::fmt::Display for Level {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Level::One => f.write_str("1"),
            Level::Two => f.write_str("2"),
        }
    }
}

pub fn download_input(day: i8) {
    let output_path = format!("input/day{day}.txt");
    let output_path = Path::new(&output_path);
    if output_path.exists() {
        return;
    }

    if !fs::exists(INPUT_DIR).unwrap() {
        fs::create_dir(INPUT_DIR).unwrap();
    }

    let mut output = OpenOptions::new()
        .write(true)
        .create(true)
        .open(output_path)
        .unwrap();

    let token = get_token();
    let res = ureq::get(format!("{AOC_2015_BASE_URL}/day/{day}/input"))
        .header("Cookie", format!("session={token}"))
        .call()
        .unwrap()
        .body_mut()
        .read_to_string()
        .unwrap();

    write!(&mut output, "{res}").unwrap();
}

pub fn submit(day: u8, level: Level, answer: String) {
    let url = format!("{AOC_2015_BASE_URL}/day/{day}/answer");
    let token = get_token();

    let level = match level {
        Level::One => "1",
        Level::Two => "2",
    };
    let form = [("level", level), ("answer", &answer)];

    let res = ureq::post(url)
        .header("Cookie", format!("session={token}"))
        .send_form(form)
        .unwrap()
        .body_mut()
        .read_to_string()
        .unwrap();

    println!("Got response: {res}");
}

pub fn load_input(day: i8) -> String {
    let input = File::open(format!("{INPUT_DIR}/day{day}.txt")).unwrap();
    let mut reader = BufReader::new(input);
    let mut out = String::new();
    reader.read_to_string(&mut out).unwrap();

    out
}

fn get_token() -> String {
    const AOC_SESSION: &str = "AOC_SESSION";

    match env::var(AOC_SESSION) {
        Ok(val) => val,
        Err(e) => {
            eprintln!("Failed to parse AOC_SESSION env: {e}");
            std::process::exit(1)
        }
    }
}
