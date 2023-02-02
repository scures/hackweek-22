# HackWeek 22 - 🦀 Rust
Stateless applications on Kubernetes with Rust that will periodically check the metrics of the cluster and report back with information about the nodes metrics.


This HackWeek objectives:
  - learn about Rust's concepts, syntax & ecosystem
  - learn more about K8s API and CRDs
  - build a simple operator (?) in Rust


### To improve:
  ☐ Handle nodes status when it's not ready and we cannot fetch the metrics
  ☐ Handle the Local cluster since metrics cannot be fetched from it
  ✔ Group metrics nodes by cluster (?)
  ☐ Add interval to refresh metrics


#### Deployment:
  ☐ Create a Dockerfile for the application
  ☐ Publish the Dockerfile to a public repository
  ☐ Create the Kubernetes deployment manifest


### Resources:
- [Rust Book](https://rust-book.cs.brown.edu/experiment-intro.html)
