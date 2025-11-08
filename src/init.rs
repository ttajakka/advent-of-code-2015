use std::{
    fs::{self, OpenOptions},
    io::Write,
};

use tera::{Context, Tera};

pub fn init(first_day: u8, last_day: u8, modify_lib: bool) {
    let tera = match Tera::new("templates/*.rs.tera") {
        Ok(t) => t,
        Err(e) => {
            println!("Parsing error(s): {}", e);
            std::process::exit(1);
        }
    };

    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open("src/lib.rs")
        .unwrap();
    let mut context = Context::new();

    for day in first_day..=last_day {
        context.insert("day", &day);
        let a = tera.render("day.rs.tera", &context).unwrap();

        fs::write(format!("src/day{day}.rs"), a).unwrap();

        if modify_lib {
            writeln!(&mut file, "pub mod day{};", day).unwrap();
        }
    }
}

pub fn tera_context() -> (Tera, Context) {
    let tera = match Tera::new("templates/*.rs.tera") {
        Ok(t) => t,
        Err(e) => {
            println!("Parsing error(s): {}", e);
            std::process::exit(1);
        }
    };

    let context = Context::new();

    (tera, context)
}

pub fn init_day(day: u8, t: Tera, mut context: Context) {
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open("src/lib.rs")
        .unwrap();
    writeln!(&mut file, "pub mod day{};", day).unwrap();

    context.insert("day", &day);
    let a = t.render("day.rs.tera", &context).unwrap();

    fs::write(format!("src/day{day}.rs"), a).unwrap();

}
