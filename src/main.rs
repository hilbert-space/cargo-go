extern crate core;
#[macro_use]
extern crate log;
use crate::client::Client;
use anyhow::anyhow;
use clap::Parser;
use std::fmt;
use std::fmt::Formatter;

mod client;
mod response;

#[derive(Debug)]
enum Destination {
    Crates,
    Documentation,
    Homepage,
    Repository,
}

impl fmt::Display for Destination {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Destination::Crates => write!(f, "Crates.io page"),
            Destination::Documentation => write!(f, "Documentation page"),
            Destination::Homepage => write!(f, "Homepage"),
            Destination::Repository => write!(f, "Repository"),
        }
    }
}

fn main() -> anyhow::Result<()> {
    pretty_env_logger::try_init()?;
    let args = Go::try_parse()?;
    args.run()?;
    Ok(())
}

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
            Destination::Crates => {
                paris::info!("Opening crates page for: {}", self.name);
                rel.crates()
            }
            Destination::Documentation => {
                paris::info!("Opening docs page for: {}", self.name);
                rel.documentation()
            }
            Destination::Homepage => {
                paris::info!("Opening homepage for: {}", self.name);
                rel.homepage()
            }
            Destination::Repository => {
                paris::info!("Opening the repository: {}", self.name);
                rel.crate_field.repository
            }
        };
        if let Err(e) = webbrowser::open(&url) {
            paris::error!("Failed to open the browser on {} due to {}", url, e);
            Err(e.into())
        } else {
            paris::success!("Opened the {} for {}", self.destination, self.name);
            Ok(())
        }
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
