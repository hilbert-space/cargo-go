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
        let res = self.0.get(url).send()?;
        let res_text = res.text()?;
        let jd = &mut serde_json::Deserializer::from_str(&res_text);
        return match serde_path_to_error::deserialize(jd) {
            Ok(s) => Ok(s),
            Err(e) => {
                error!("Error while deserializing {}\n", e);
                error!("Path to error: {}\n", e.path().to_string());
                error!("Raw data: {}\n", res_text);
                Err(e.into())
            }
        };
    }
}
