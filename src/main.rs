use serde_json::Value;
use std::env;
mod utils;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let get_clusters = fetch("v1/management.cattle.io.clusters").await?;

    println!("{}", get_clusters);
    Ok(())
}

async fn fetch(endpoint: &str) -> Result<Value, reqwest::Error> {
    let rancher_url = env::var("RANCHER_URL").expect("Error: VAR_NAME not found");
    let bearer_token = env::var("RANCHER_TOKEN").expect("Error: VAR_NAME not found");

    print!("{}{}", rancher_url, bearer_token);

    let url = format!("{}{}", rancher_url, endpoint);

    let response = reqwest::Client::new()
        .get(url)
        .bearer_auth(bearer_token)
        .send()
        .await?
        .text()
        .await?;

    let parsed_json = utils::parse_json(&response).unwrap();
    // Return the clusters as parsed json
    Ok(parsed_json)
}
