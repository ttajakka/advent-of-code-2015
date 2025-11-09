use std::fs;

use tera::{Context, Tera};

pub fn init(first_day: u8, last_day: u8) {
    let (tera, mut context) = get_tera_context();

    for day in first_day..=last_day {
        context.insert("day", &day);
        let rendered = tera.render("day.rs.tera", &context).unwrap();

        fs::write(format!("src/day{day}.rs"), rendered).unwrap();
    }
}

pub fn make_lib(num_days: u8) {
    let (tera, mut context) = get_tera_context();

    let days: Vec<_> = (1..=num_days).collect();
    context.insert("days", &days);
    let rendered = tera.render("lib.rs.tera", &context).unwrap();
    fs::write(format!("src/lib.rs"), rendered).unwrap();
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
    context.insert("day", &day);
    let a = t.render("day.rs.tera", &context).unwrap();

    fs::write(format!("src/day{day}.rs"), a).unwrap();
}

fn get_tera_context() -> (tera::Tera, tera::Context) {
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
