use crate::response::Response;
use anyhow::Result;

use reqwest::header::HeaderMap;

const CRATES_URL: &str = "https://crates.io/api/v1/crates";

pub struct Client(reqwest::blocking::Client);

impl Client {
    pub fn new() -> Result<Self> {
        let mut heads = HeaderMap::new();
        heads.insert("User-Agent", "cargo-go".parse()?);
        let client = reqwest::blocking::Client::builder()
            .default_headers(heads)
            .build()?;
        Ok(Self(client))
    }
    pub fn new_load(&self, name: &str) -> Result<Response> {
        let url = format!("{}/{}", CRATES_URL, name);
        let res = self.0.get(url).send()?.json();
        if let Err(e) = &res {
            error!("Error while deserializing {}\n", e);
        }
        Ok(res?)
    }
}
