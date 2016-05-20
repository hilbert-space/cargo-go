extern crate curl;
extern crate regex;

use std::{env, process};

macro_rules! ok(
    ($result:expr) => (match $result {
        Ok(result) => result,
        Err(error) => raise!("{}", error),
    });
);

macro_rules! raise(
    ($message:expr) => (return Err(format!("{}", $message)));
    ($($argument:tt)*) => (raise!(format!($($argument)*)));
);

mod find;
mod load;
mod open;

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
        process::exit(1);
    }
}

fn run() -> Result<(), String> {
    let path = match parse() {
        None | Some((_, Destination::Unknown)) => raise!("do not know where to go"),
        Some((name, Destination::Crates)) => {
            format!("https://crates.io/crates/{}", name)
        },
        Some((name, Destination::Documentation)) => {
            match ok!(find::find("documentation", &try!(load::load(&name)))) {
                Some(path) => path,
                _ => raise!("cannot find the documentation"),
            }
        },
        Some((name, Destination::Homepage)) => {
            match ok!(find::find("homepage", &try!(load::load(&name)))) {
                Some(path) => path,
                _ => raise!("cannot find the homepage"),
            }
        },
        Some((name, Destination::Repository)) => {
            match ok!(find::find("repository", &try!(load::load(&name)))) {
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
        "c" | "crate" | "crates" | "crates.io" | "" => Destination::Crates,
        "d" | "doc" | "docs" | "documentation" => Destination::Documentation,
        "h" | "home" | "homepage" | "page" | "web" | "website" => Destination::Homepage,
        "r" | "git" | "rep" | "repository" => Destination::Repository,
        _ => Destination::Unknown,
    };
    Some((name, destination))
}
