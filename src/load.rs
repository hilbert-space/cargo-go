use std::io::prelude::*;
use hyper::Client;

const BASE: &'static str = "https://crates.io/api/v1/crates";

pub fn load(name: &str) -> Result<String, String> {
    let path = format!("{}/{}", BASE, name);

    let client = Client::new();
    let mut response = ok!(client.get(&path).send());
    let mut body = String::new();
    ok!(response.read_to_string(&mut body));

    Ok(body)
}
