# Unofficial Rust Test Container For Redpanda

[![github action](https://github.com/milenkovicm/testcontainers-redpanda-rs/actions/workflows/basic.yml/badge.svg)](https://github.com/milenkovicm/testcontainers-redpanda-rs/actions/workflows/basic.yml)
[![Crates.io](https://img.shields.io/crates/v/testcontainers-redpanda-rs)](https://crates.io/crates/testcontainers-redpanda-rs)
[![Crates.io](https://img.shields.io/crates/d/testcontainers-redpanda-rs)](https://crates.io/crates/testcontainers-redpanda-rs)

Unofficial testcontainer for [Redpanda](https://redpanda.com). Redpanda is a simple, powerful, and cost-efficient streaming data platform that is compatible with Kafka APIs but much less complex, faster and more affordable.

Add dependency:

```toml
testcontainers-redpanda-rs = { version = "0.7" }
```

Create and run redpanda container:

```rust, no_run
use testcontainers_redpanda_rs::*;

#[tokio::main]
async fn main() {
    let container = Redpanda::latest();

    let server_node = container.start().await.unwrap();
    let bootstrap_servers = format!("localhost:{}", server_node.get_host_port_ipv4(REDPANDA_PORT).await.unwrap());
    // if topic has only one partition this part is optional
    // it will be automatically created when client connects
    server_node.exec(Redpanda::cmd_create_topic("test_topic", 3)).await.unwrap();

    println!("Redpanda server: {}", bootstrap_servers);
}
```

Note about version compatibility:

- `0.7.x` supports `testcontainers` `0.21`
- `0.6.x` supports `testcontainers` `0.20`
- `0.5.x` supports `testcontainers` `0.19`
- `0.4.x` supports `testcontainers` `0.18`
- `0.3.x` supports `testcontainers` `0.17`
- `0.2.x` supports `testcontainers` `0.16`
- `0.1.x` supports `testcontainers` `0.15`
