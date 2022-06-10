use std::collections::HashMap;
use std::fs;

use hyper::header::AUTHORIZATION;
use hyper::HeaderMap;

use serde_json::Value;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let token = fs::read_to_string("/home/dougfort/.config/twitterv2/token")?;
    let token = token.trim();
    let mut headers = HeaderMap::new();
    headers.insert(AUTHORIZATION, format!("Bearer {}", token).parse()?);

    let client = reqwest::Client::new();
    let text = client
        .get("https://api.twitter.com/2/tweets/search/recent?query=aaa")
        .headers(headers)
        .send()
        .await?
        .error_for_status()?
        .text()
        .await?;
    let v: Value = serde_json::from_str(&text)?;

    println!(
        "result-count: {}; oldest-id: {}; newest-id: {}; next-token: {}",
        v["meta"]["result_count"],
        v["meta"]["oldest_id"],
        v["meta"]["newest_id"],
        v["meta"]["next-token"]
    );
    if let Value::Array(data_vec)= &v["data"] {
        data_vec.iter().for_each(|item| {
            println!("{} {}", item["id"], item["text"]);
        });  
    }

    Ok(())
}
