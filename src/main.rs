use std::collections::HashMap;
use std::sync::Arc;
use reqwest::cookie::Jar;
use reqwest::Url;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cookie = "foo=bar; Domain=yolo.local";
    let url = "https://yolo.local".parse::<Url>().unwrap();

    let jar = Jar::default();
    jar.add_cookie_str(cookie, &url);

    let client = reqwest::Client::new();

    let resp = client.get("https://httpbin.org/ip")
        .send()
        .await?
        .json::<HashMap<String, String>>()
        .await?;
    println!("{:#?}", resp);
    Ok(())
}
