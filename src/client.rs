use crate::response::Response;
use anyhow::Result;
use hyper::client::HttpConnector;
use hyper_rustls::HttpsConnector;

const CRATES_URL: &str = "https://crates.io/api/v1/crates";

pub struct Client(hyper::Client<HttpsConnector<HttpConnector>>);

impl Client {
    pub fn new() -> Self {
        let https = hyper_rustls::HttpsConnectorBuilder::new()
            .with_native_roots()
            .https_only()
            .enable_http1()
            .enable_http2()
            .build();
        let client = hyper::Client::builder().build(https);
        Self(client)
    }
    pub async fn new_load(&self, name: &str) -> Result<Response> {
        let url = format!("{}/{}", CRATES_URL, name);
        let resp = self.0.get(url.parse()?).await?;
        Response::from_hyper(resp).await
    }
}
