use serde::{Deserialize, Serialize};

mod utils;

#[derive(Debug, Deserialize, Serialize)]
pub struct Usage {
    cpu: String,
    memory: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Node {
    name: String,
    usage: Usage,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ClusterMetrics {
    id: String,
    nodes: Vec<Node>,
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

        println!("Clusters List: {:?}", ids);

        Ok(ids)
    }

    // Store the cluster ids in a vector
    let ids = get_ids().await.unwrap();

    let mut overall = Vec::new();

    // Iterate over the cluster ids and get the metrics for each cluster
    for id in ids {
        // Get the nods from cluster
        let get_cluster_nodes =
            utils::fetch(&format!("/k8s/clusters/{}/v1/metrics.k8s.io.nodes", id))
                .await
                .unwrap();

        let mut v = Vec::new();

        for cluster_nodes in &get_cluster_nodes["data"].as_array() {
            // println!("metric {:?}", cluster_nodes);

            for node in cluster_nodes.iter() {
                let node_name = node.get("id").unwrap().to_string();
                let x = node
                    .get("usage")
                    .map(|usage| usage.clone())
                    .unwrap()
                    .to_string();

                let metrics = utils::extract_data(&x);
                // println!("🦀 ID: {}, Metrics: {:?}", node_name, node);

                // TODO: Set a node with a status NOT-READY if the metric response is empty
                v.push(Node {
                    name: node_name,
                    usage: metrics,
                });
            }
        }

        // For each cluster, store a new ClusterMetrics struct into the overall vec
        overall.push(ClusterMetrics {
            id: id.to_string(),
            nodes: v,
        });

        // println!("{:?}", &v);
    }

    // convert the overall vec to json
    let json = serde_json::to_string(&overall).unwrap();

    println!("{:?}", json)

    // TODO: Send the overall vec to Rancher as CRD
}
