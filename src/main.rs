extern crate curl;
extern crate regex;

use std::env;

macro_rules! ok(
    ($result:expr) => (match $result {
        Ok(result) => result,
        Err(error) => raise!("{}", error),
    });
);

macro_rules! raise(
    ($message:expr) => (return Err($message.to_string()));
    ($($argument:tt)*) => (raise!(format!($($argument)*)));
);

mod find;
mod load;
mod open;

const BASE: &'static str = "https://crates.io/crates";

enum Destination {
    Crates,
    Documentation,
    Homepage,
    Repository,
    Unknown,
}

fn main() {
    if let Err(error) = run() {
        println!("Error: {}.", error);
    }
}

fn run() -> Result<(), String> {
    let path = match parse() {
        None | Some((_, Destination::Unknown)) => raise!("do not know where to go"),
        Some((name, Destination::Crates)) => format!("{}/{}", BASE, name),
        Some((name, Destination::Documentation)) => {
            match ok!(find::find("documentation", &load::load(&name)?)) {
                Some(path) => path,
                _ => raise!("cannot find the documentation"),
            }
        },
        Some((name, Destination::Homepage)) => {
            match ok!(find::find("homepage", &load::load(&name)?)) {
                Some(path) => path,
                _ => raise!("cannot find the home page"),
            }
        },
        Some((name, Destination::Repository)) => {
            match ok!(find::find("repository", &load::load(&name)?)) {
                Some(path) => path,
                _ => raise!("cannot find the repository"),
            }
        },
    };
    open::open(&path)
}

fn parse() -> Option<(String, Destination)> {
    let mut arguments = env::args().collect::<Vec<_>>();
    let (name, destination) = match arguments.len() {
        0...2 => return None,
        3 => (arguments.remove(2), String::new()),
        _ => (arguments.remove(2), arguments.remove(2).to_lowercase()),
    };
    let destination = match &*destination {
        "c" | "" | "crate" | "crates" | "crates.io" => Destination::Crates,
        "d" | "doc" | "docs" | "documentation" => Destination::Documentation,
        "h" | "home" | "homepage" | "page" | "web" | "website" => Destination::Homepage,
        "r" | "git" | "rep" | "repo" | "repository" => Destination::Repository,
        _ => Destination::Unknown,
    };
    Some((name, destination))
}
