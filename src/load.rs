use curl::http;

const BASE: &'static str = "https://crates.io/api/crates";

macro_rules! ok(
    ($result:expr) => (
        match $result {
            Ok(result) => result,
            Err(error) => return Err(format!("{}", error)),
        }
    );
);

pub fn load(name: &str) -> Result<String, String> {
    let path = format!("{}/{}", BASE, name);
    let response = ok!(http::handle().get(path).follow_redirects(true).exec());
    Ok(ok!(String::from_utf8(response.get_body().to_vec())))
}
