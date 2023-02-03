# HackWeek 22 - 🦀 Rust
Stateless applications on Kubernetes with Rust that will periodically check the metrics of the cluster and report back with information about the nodes metrics.


This HackWeek objectives:
  - learn about Rust's concepts, syntax & ecosystem
  - learn more about K8s API and CRDs
  - build a simple operator (?) in Rust


### How to run:
You'll need a Rancher cluster to run this application. There's a GithubAction that will build & deploy the Docker Image to the registry; https://hub.docker.com/r/scures/hackweek

Some environment variables are required to run the application:
- `RANCHER_URL` - URL to the Rancher cluster
- `RANCHER_TOKEN` - Token to access the Rancher API


Apply the Kubernetes manifest to deploy the application:
```bash
kubectl apply -f k8s/deployment.yaml
```

---
### To improve:
Some things that need to be improved:
- [ ] Handle nodes status when it's not ready and we cannot fetch the metrics
- [ ] Handle the Local cluster since metrics cannot be fetched from it
- [x] Group metrics nodes by cluster (?)
- [x] Add interval to refresh metrics


#### Deployment:
Things to do to deploy the application in a Cluster
- [x] Create a Dockerfile tp build the application
- [x] Publish the Docker Image to Docker Registry
- [x] Create the Kubernetes deployment manifest
- [ ] Create and apply CRD


### Resources:
- [Rust Book](https://rust-book.cs.brown.edu/experiment-intro.html)
