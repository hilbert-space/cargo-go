use serde_json::{from_str, Value};

pub fn find(key: &str, json: &str) -> Result<Option<String>, String> {
    let value: Value = ok!(from_str(json));
    Ok(value.lookup("crate")
            .and_then(|c| c.lookup(key))
            .and_then(Value::as_string)
            .map(ToOwned::to_owned))
}

#[cfg(test)]
mod tests {
    const RESPONSE: &'static str = include_str!("../tests/fixtures/cargo.json");

    #[test]
    fn find() {
        assert_eq!(super::find("homepage", RESPONSE),
                   Ok(Some("https://crates.io".into())));
    }
}
