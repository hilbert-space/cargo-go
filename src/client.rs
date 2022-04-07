use crate::response::Response;
use anyhow::{Context, Result};
use hyper::client::HttpConnector;
use hyper_rustls::HttpsConnector;
use reqwest::header::HeaderMap;

const CRATES_URL: &str = "https://crates.io/api/v1/crates";

pub struct Client(reqwest::blocking::Client);

impl Client {
    pub fn new() -> Result<Self> {
        let mut heads = HeaderMap::new();
        heads.insert("User-Agent", "cargo-go".parse()?);
        let client = reqwest::blocking::Client::builder().default_headers(heads).build()?;
        Ok(Self(client))
    }
    pub fn new_load(&self, name: &str) -> Result<Response> {
        let url = format!("{}/{}", CRATES_URL, name);
        let res = self.0.get(url).send()?;
        let text = res.text()?;
        let res2: serde_json::Result<Response> = serde_json::from_str(&text);
        if let Err(e) = &res2 {
            error!("Error while deserializing {}\nHere's the raw data: {}", e, text);
        }
        Ok(res2?)
    }
}
