use curl::easy::Easy;

const CRATES_URL: &'static str = "https://crates.io/api/v1/crates";

pub fn load(name: &str) -> Result<String, String> {
    let url = format!("{}/{}", CRATES_URL, name);
    let mut data = Vec::new();
    let mut handle = Easy::new();
    ok!(handle.url(&url));
    ok!(handle.follow_location(true));
    {
        let mut transfer = handle.transfer();
        ok!(transfer.write_function(|new_data| {
            data.extend_from_slice(new_data);
            Ok(new_data.len())
        }));
        ok!(transfer.perform());
    }
    Ok(ok!(String::from_utf8(data)))
}
