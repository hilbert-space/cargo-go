use regex::Regex;

pub fn find(key: &str, json: &str) -> Result<Option<String>, String> {
    let pattern = ok!(Regex::new(&format!(r#""{}":"([^"]*)""#, key)));
    Ok(pattern
        .captures(json)
        .and_then(|captures| captures.get(1))
        .map(|_match| _match.as_str().into()))
}

#[cfg(test)]
mod tests {
    const RESPONSE: &str = include_str!("../tests/fixtures/cargo.json");

    #[test]
    fn find() {
        assert_eq!(
            super::find("homepage", RESPONSE),
            Ok(Some("https://crates.io".into()))
        );
    }
}
