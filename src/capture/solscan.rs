use std::error::Error;

use reqwest::header::{HeaderMap, HeaderValue};

use super::Capture;

pub struct SolscanCapture {
    url: String,
}

impl SolscanCapture {
    pub fn new(address: String) -> Self {
        Self {
            url: format!(
                "https://api.solscan.io/v2/account/v2/tokens?address={}",
                address
            ),
        }
    }
}

impl Capture for SolscanCapture {
    fn get(&self) -> Result<String, Box<dyn Error>> {
        let mut headers = HeaderMap::new();
        headers.insert("authority", HeaderValue::from_static("api.solscan.io"));
        headers.insert("method", HeaderValue::from_static("GET"));
        headers.insert("scheme", HeaderValue::from_static("https"));
        headers.insert(
            "accept",
            HeaderValue::from_static("application/json, text/plain, */*"),
        );
        headers.insert("cache-control", HeaderValue::from_static("no-cache"));
        headers.insert("origin", HeaderValue::from_static("https://solscan.io"));
        headers.insert("pragma", HeaderValue::from_static("no-cache"));
        headers.insert("referer", HeaderValue::from_static("https://solscan.io/"));
        headers.insert(
            "sol-au",
            HeaderValue::from_static("h=OmOdf9GNGMgzB3N91lVInHyyB9dls02fKOk8srR"),
        );
        headers.insert("user-agent", HeaderValue::from_static("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/123.0.0.0 Safari/537.36"));
        let client = reqwest::blocking::Client::new();
        let data = client.get(self.url.clone()).headers(headers).send()?.text();
        Ok(data?)
    }

    fn save(&self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}
