use reqwest::header::{HeaderMap, HeaderValue};

#[tokio::main]
async fn main() {
    let url = "https://api.solscan.io/v2/account/v2/tokens?address=EuufsAetst5dj2MZYZHzSiy6DBq6Rye4j9F58SEc2yks";
    let mut headers = HeaderMap::new();
    headers.insert("authority", HeaderValue::from_static("api.solscan.io"));
    headers.insert("method", HeaderValue::from_static("GET"));
    // headers.insert(
    //     "path",
    //     HeaderValue::from_static(
    //         "/v2/account/v2/tokens?address=EuufsAetst5dj2MZYZHzSiy6DBq6Rye4j9F58SEc2yks",
    //     ),
    // );
    headers.insert("scheme", HeaderValue::from_static("https"));
    headers.insert(
        "accept",
        HeaderValue::from_static("application/json, text/plain, */*"),
    );
    // headers.insert(
    //     "accept-encoding",
    //     HeaderValue::from_static("gzip, deflate, br, zstd"),
    // );
    // headers.insert(
    //     "accept-language",
    //     HeaderValue::from_static("zh-CN,zh;q=0.9,en;q=0.8,zh-TW;q=0.7,ja;q=0.6,mt;q=0.5,ru;q=0.4"),
    // );
    headers.insert("cache-control", HeaderValue::from_static("no-cache"));
    headers.insert("origin", HeaderValue::from_static("https://solscan.io"));
    headers.insert("pragma", HeaderValue::from_static("no-cache"));
    headers.insert("referer", HeaderValue::from_static("https://solscan.io/"));
    // headers.insert(
    //     "sec-ch-ua",
    //     HeaderValue::from_static(
    //         "\"Google Chrome\";v=\"123\", \"Not:A-Brand\";v=\"8\", \"Chromium\";v=\"123\"",
    //     ),
    // );
    // headers.insert("sec-ch-ua-mobile", HeaderValue::from_static("?0"));
    // headers.insert("sec-ch-ua-platform", HeaderValue::from_static("\"macOS\""));
    // headers.insert("sec-fetch-dest", HeaderValue::from_static("empty"));
    // headers.insert("sec-fetch-mode", HeaderValue::from_static("cors"));
    // headers.insert("sec-fetch-site", HeaderValue::from_static("same-site"));
    headers.insert(
        "sol-au",
        HeaderValue::from_static("h=OmOdf9GNGMgzB3N91lVInHyyB9dls02fKOk8srR"),
    );
    headers.insert("user-agent", HeaderValue::from_static("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/123.0.0.0 Safari/537.36"));
    let client = reqwest::Client::new();

    let res = client.get(url).headers(headers).send().await.unwrap();
    println!("{}", res.text().await.unwrap());
}
