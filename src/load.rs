use crate::response::Response;
use curl::easy::Easy;
use hyper_rustls::HttpsConnectorBuilder;

const CRATES_URL: &str = "https://crates.io/api/v1/crates";

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

pub async fn new_load(name: &str) -> anyhow::Result<Response> {
    let https = HttpsConnectorBuilder::new()
        .with_native_roots()
        .https_only()
        .enable_http2()
        .build();
    let client: hyper::Client<_, hyper::Body> = hyper::Client::builder().build(https);
    let url = format!("{}/{}", CRATES_URL, name);
    Response::from_hyper(client.get(url.parse()?).await?).await
}
