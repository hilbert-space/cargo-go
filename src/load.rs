use hyper::Client;
use std::io::Read;

const BASE: &'static str = "https://crates.io/api/v1/crates";

pub fn load(name: &str) -> Result<String, String> {
    let mut response = ok!(Client::new().get(&format!("{}/{}", BASE, name)).send());
    let mut body = String::new();
    ok!(response.read_to_string(&mut body));
    Ok(body)
}
