use serde::Deserialize;

mod utils;

#[derive(Debug, Deserialize)]
pub struct Usage {
    cpu: String,
    memory: String,
}

#[derive(Debug, Deserialize)]
pub struct Node {
    name: String,
    usage: Usage,
}

#[tokio::main]
async fn main() {
    // Fetch the cluster ids
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

    // Store the cluster ids in a vector
    let ids = get_ids().await.unwrap();

    let mut overall: Vec<Node> = Vec::new();

    // Iterate over the cluster ids and get the metrics for each cluster
    for id in ids {
        // Get the metrics for each cluster id
        let get_metrics = utils::fetch(&format!("/k8s/clusters/{}/v1/metrics.k8s.io.nodes", id))
            .await
            .unwrap();

        for _metric in &get_metrics["data"].as_array() {
            let x = _metric[0]
                .get("usage")
                .map(|usage| usage.clone())
                .unwrap()
                .to_string();

            let metrics = utils::extract_data(&x);
            println!("ID: {}, Metrics: {:?}", id, metrics);

            overall.push(Node {
                name: id.to_string(),
                usage: metrics,
            });
        }
    }

    // Print overall vec of nodes
    println!("{:?}", overall);

    // TODO: Send the overall vec to Rancher as CRD
}
