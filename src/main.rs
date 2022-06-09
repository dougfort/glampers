use std::collections::HashMap;
use std::fs;

use hyper::HeaderMap;
use hyper::header::AUTHORIZATION;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let token = fs::read_to_string("/home/dougfort/.config/twitterv2/token")?;
    let token = token.trim();
    let mut headers = HeaderMap::new();
    headers.insert(AUTHORIZATION,format!("Bearer {}",token).parse()?);
            
    let client = reqwest::Client::new();
    let resp = client
        .get("https://api.twitter.com/2/tweets/search/recent?query=aaa")
        .headers(headers)
        .send()
        .await?   
        .error_for_status()?     
//        .json::<HashMap<String, String>>()
        .text()
        .await?;
    println!("{:?}", resp);
    Ok(())
}
