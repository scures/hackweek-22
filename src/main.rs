mod utils;

#[tokio::main]
async fn main() {
    async fn get_ids() -> Result<Vec<String>, reqwest::Error> {
        let mut ids = Vec::new();
        let get_clusters = utils::fetch("v1/management.cattle.io.clusters").await?;

        if let Some(data) = get_clusters["data"].as_array() {
            for item in data {
                if let Some(id) = item["id"].as_str() {
                    // Ignore the local cluster since the k8s/clusters/local/v1/metrics.k8s.io.nodes won't work (?)
                    if id == "local" {
                        continue;
                    }

                    ids.push(id.to_string())
                }
            }
        }

        println!("{:?}", ids);

        Ok(ids)
    }

    let ids = get_ids().await.unwrap();

    for id in ids {
        // Get the metrics for each cluster id
        let get_metrics = utils::fetch(&format!("/k8s/clusters/{}/v1/metrics.k8s.io.nodes", id))
            .await
            .unwrap();

        // TODO: Process data

        println!("ID: {}: {}", id, get_metrics);
    }
}
