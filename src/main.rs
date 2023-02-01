mod utils;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let get_clusters = utils::fetch("v1/management.cattle.io.clusters").await?;

    println!("{}", get_clusters);
    Ok(())
}
