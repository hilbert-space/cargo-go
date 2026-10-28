#![deny(clippy::missing_docs_in_private_items)]

extern crate core;
#[macro_use]
extern crate log;
use crate::client::Client;
use anyhow::anyhow;
use clap::AppSettings;
use clap::ArgEnum;

use clap::Parser;

use clap_verbosity_flag::{ErrorLevel, Verbosity};

use std::fmt;
use std::fmt::Formatter;

/// HTTP access to crates.io.
mod client;
/// Crates.io response models and page URLs.
mod response;

#[derive(Debug, ArgEnum, Clone)]
enum Destination {
    /// Open the crate's crates.io page.
    #[clap(aliases(["c", "", "crate", "crates", "crates.io"]))]
    Crates,
    /// Open the crate's documentation page.
    #[clap(aliases(["d", "doc", "docs", "documentation"]))]
    Documentation,
    /// Open the crate's homepage.
    #[clap(aliases(["h", "home", "homepage", "page", "web", "website"]))]
    Homepage,
    /// Open the crate's source repository.
    #[clap(aliases(["r", "git", "rep", "repo", "repository"]))]
    Repository,
}

impl fmt::Display for Destination {
    /// Formats the destination for user-facing messages.
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Destination::Crates => write!(f, "Crates.io page"),
            Destination::Documentation => write!(f, "Documentation page"),
            Destination::Homepage => write!(f, "Homepage"),
            Destination::Repository => write!(f, "Repository"),
        }
    }
}

/// Parses arguments and runs the command-line application.
fn main() -> anyhow::Result<()> {
    human_panic::setup_panic!();
    let args: Go = Go::try_parse()?;
    pretty_env_logger::formatted_builder()
        .filter_level(args.verbose.log_level_filter())
        .try_init()?;
    args.run()?;
    Ok(())
}

#[derive(Parser)]
#[clap(global_setting(AppSettings::DeriveDisplayOrder))]
struct Go {
    /// Page to open for the package.
    #[clap(arg_enum)]
    destination: Destination,
    /// Crate name to look up.
    name: String,
    /// Controls the amount of command output.
    #[clap(flatten)]
    verbose: Verbosity<ErrorLevel>,
}

impl Go {
    /// Looks up the crate and opens the selected page.
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
                match rel.homepage() {
                    Some(s) => s,
                    None => {
                        paris::error!("No homepage found for: {}", self.name);
                        return Err(anyhow!("No homepage found"));
                    }
                }
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
