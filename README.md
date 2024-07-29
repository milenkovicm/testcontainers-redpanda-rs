# Unofficial Rust Test Container For Redpanda

[![github action](https://github.com/milenkovicm/testcontainers-redpanda-rs/actions/workflows/basic.yml/badge.svg)](https://github.com/milenkovicm/testcontainers-redpanda-rs/actions/workflows/basic.yml)
[![Crates.io](https://img.shields.io/crates/v/testcontainers-redpanda-rs)](https://crates.io/crates/testcontainers-redpanda-rs)
[![Crates.io](https://img.shields.io/crates/d/testcontainers-redpanda-rs)](https://crates.io/crates/testcontainers-redpanda-rs)

Unofficial testcontainer for [Redpanda](https://redpanda.com). Redpanda is a simple, powerful, and cost-efficient streaming data platform that is compatible with Kafka APIs but much less complex, faster and more affordable.

Note:

- version `0.6.x` supports `testcontainer` `0.20`
- version `0.5.x` supports `testcontainer` `0.19`
- version `0.4.x` supports `testcontainer` `0.18`
- version `0.3.x` supports `testcontainer` `0.17`
- version `0.2.x` supports `testcontainer` `0.16`
- version `0.1.x` supports `testcontainer` `0.15`

Add dependency:

```toml
testcontainers-redpanda-rs = { version = "0.6" }
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

Limitations:

- It will use default kafka ports and only single test can run  on given host.
