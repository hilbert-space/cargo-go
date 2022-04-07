extern crate core;
extern crate curl;
extern crate regex;
#[macro_use] extern crate log;
use crate::client::Client;
use anyhow::{anyhow, Context};
use clap::Parser;
use log::LevelFilter;
// use std::env;

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

mod client;
mod find;
mod load;
mod open;
mod response;

const BASE: &str = "https://crates.io/crates";

enum Destination {
    Crates,
    Documentation,
    Homepage,
    Repository,
}

fn main() -> anyhow::Result<()> {
    pretty_env_logger::try_init()?;
    let args = Go::try_parse()?;
    args.run()?;
    Ok(())
}

// async fn new_run() -> anyhow::Result<()> {
//     let client = client::Client::new();
//     let path = match parse() {
//         None | Some((_, Destination::Unknown)) => raise!("do not know where to go"),
//         Some((name, Destination::Crates)) => format!("{}/{}", BASE, name),
//         Some((name, Destination::Documentation)) => {
//             match ok!(find::find("documentation", &load::load(&name)?)) {
//                 Some(path) => path,
//                 _ => raise!("cannot find the documentation"),
//             }
//         }
//         Some((name, Destination::Homepage)) => {
//             match ok!(find::find("homepage", &load::load(&name)?)) {
//                 Some(path) => path,
//                 _ => raise!("cannot find the home page"),
//             }
//         }
//         Some((name, Destination::Repository)) => {
//             match ok!(find::find("repository", &load::load(&name)?)) {
//                 Some(path) => path,
//                 _ => raise!("cannot find the repository"),
//             }
//         }
//         Ok(())
//
//     }
//
// }

#[derive(Parser)]
struct Go {
    #[clap(parse(try_from_str = parse_dest))]
    destination: Destination,
    name: String,
}

impl Go {
    pub fn run(self) -> anyhow::Result<()> {
        let client = Client::new()?;
        let rel = client.new_load(&self.name)?;
        let url = match self.destination {
            Destination::Crates => rel.crates(),
            Destination::Documentation => rel.documentation(),
            Destination::Homepage => rel.homepage(),
            Destination::Repository => rel.crate_field.repository,
        };
        webbrowser::open(&url).context("Failed to open the browser")
    }
}

fn parse_dest(input: &str) -> anyhow::Result<Destination> {
    let destination = match input {
        "c" | "" | "crate" | "crates" | "crates.io" => Destination::Crates,
        "d" | "doc" | "docs" | "documentation" => Destination::Documentation,
        "h" | "home" | "homepage" | "page" | "web" | "website" => Destination::Homepage,
        "r" | "git" | "rep" | "repo" | "repository" => Destination::Repository,
        _ => return Err(anyhow!("Wrong destination")),
    };
    Ok(destination)
}

// fn run() -> Result<(), String> {
//     let client = client::Client::new();
//     let path = match parse() {
//         None | Some((_, Destination::Unknown)) => raise!("do not know where to go"),
//         Some((name, Destination::Crates)) => format!("{}/{}", BASE, name),
//         Some((name, Destination::Documentation)) => {
//             match ok!(find::find("documentation", &load::load(&name)?)) {
//                 Some(path) => path,
//                 _ => raise!("cannot find the documentation"),
//             }
//         }
//         Some((name, Destination::Homepage)) => {
//             match ok!(find::find("homepage", &load::load(&name)?)) {
//                 Some(path) => path,
//                 _ => raise!("cannot find the home page"),
//             }
//         }
//         Some((name, Destination::Repository)) => {
//             match ok!(find::find("repository", &load::load(&name)?)) {
//                 Some(path) => path,
//                 _ => raise!("cannot find the repository"),
//             }
//         }
//     };
//     open::open(&path)
// }

// fn parse() -> Option<(String, Destination)> {
//     let mut arguments = env::args().collect::<Vec<_>>();
//     let (name, destination) = match arguments.len() {
//         0..=2 => return None,
//         3 => (arguments.remove(2), String::new()),
//         _ => (arguments.remove(2), arguments.remove(2).to_lowercase()),
//     };
//     let destination = match &*destination {
//         "c" | "" | "crate" | "crates" | "crates.io" => Destination::Crates,
//         "d" | "doc" | "docs" | "documentation" => Destination::Documentation,
//         "h" | "home" | "homepage" | "page" | "web" | "website" => Destination::Homepage,
//         "r" | "git" | "rep" | "repo" | "repository" => Destination::Repository,
//         _ => Destination::Unknown,
//     };
//     Some((name, destination))
// }
