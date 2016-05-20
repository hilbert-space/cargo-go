use serde_json::{Value, from_str};

pub fn find(key: &str, json: &str) -> Result<Option<String>, String> {
    Ok(ok!(from_str::<Value>(json)).lookup("crate")
                                   .and_then(|c| c.lookup(key))
                                   .and_then(Value::as_string)
                                   .map(ToOwned::to_owned))
}

#[cfg(test)]
mod tests {
    const RESPONSE: &'static str = include_str!("../tests/fixtures/cargo.json");

    #[test]
    fn find() {
        assert_eq!(super::find("homepage", RESPONSE), Ok(Some("https://crates.io".into())));
    }
}
