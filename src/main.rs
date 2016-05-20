use std::{env, process};

mod find;
mod load;
mod open;

enum Destination {
    Crates,
    Unknown,
}

fn main() {
    let path = match parse() {
        None | Some((_, Destination::Unknown)) => error("do not know where to go"),
        Some((name, Destination::Crates)) => format!("https://crates.io/crates/{}", name),
    };
    if let Err(_) = open::open(&path) {
        error(&format!("failed to go to {:?}", path));
    }
}

fn parse() -> Option<(String, Destination)> {
    let mut args = env::args().collect::<Vec<_>>();
    let (name, destination) = match args.len() {
        0 | 1 => return None,
        2 => (args.remove(1), String::new()),
        _ => (args.remove(1), args.remove(1).to_lowercase()),
    };
    let destination = match &*destination {
        "" | "crates" | "crates.io" => Destination::Crates,
        _ => Destination::Unknown,
    };
    Some((name, destination))
}

fn error(message: &str) -> ! {
    println!("Error: {}.", message);
    process::exit(1);
}
