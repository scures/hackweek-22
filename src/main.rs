use serde_json::{Result as ResultJSON, Value};
use std::env;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let get_clusters = fetch("v1/management.cattle.io.clusters").await?;

    println!("{}", get_clusters);
    Ok(())
}

async fn fetch(endpoint: &str) -> Result<Value, reqwest::Error> {
    let rancher_url = env::var("RANCHER_URL").expect("Error: VAR_NAME not found");
    let bearer_token = env::var("RANCHER_TOKEN").expect("Error: VAR_NAME not found");

    let url = format!("{}{}", rancher_url, endpoint);

    let response = reqwest::Client::new()
        .get(url)
        .bearer_auth(bearer_token)
        .send()
        .await?
        .text()
        .await?;

    let parsed_json = parse_json(&response).unwrap();
    // Return the clusters as parsed json
    Ok(parsed_json)
}

// Used to parsed the json response
fn parse_json(json_str: &str) -> ResultJSON<Value> {
    let value: Value = serde_json::from_str(json_str)?;
    Ok(value)
}
