extern crate curl;

use std::{env, process};

macro_rules! raise(
    ($message:expr) => (return Err($message.into()));
    ($($argument:tt)*) => (raise!(format!($($argument)*)));
);

mod find;
mod load;
mod open;

enum Destination {
    Crates,
    Homepage,
    Unknown,
}

fn main() {
    if let Err(error) = run() {
        println!("Error: {}.", error);
        process::exit(1);
    }
}

fn run() -> Result<(), String> {
    let path = match parse() {
        None | Some((_, Destination::Unknown)) => raise!("do not know where to go"),
        Some((name, Destination::Crates)) => {
            format!("https://crates.io/crates/{}", name)
        },
        Some((name, Destination::Homepage)) => {
            let _ = try!(load::load(&name));
            format!("https://crates.io/crates/{}", name)
        },
    };
     open::open(&path)
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
        "home" | "homepage" | "web" | "website" => Destination::Homepage,
        _ => Destination::Unknown,
    };
    Some((name, destination))
}
