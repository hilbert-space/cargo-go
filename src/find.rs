#[allow(dead_code)]
pub fn find(_: &str, _: &str) -> Option<String> {
    None
}

#[cfg(test)]
mod tests {
    const RESPONSE: &'static str = include_str!("../tests/fixtures/cargo.json");

    #[test]
    fn find() {
        assert_eq!(super::find("homepage", RESPONSE), None);
    }
}
