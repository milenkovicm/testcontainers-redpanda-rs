# Unofficial Rust Test Container For Redpanda

[![github action](https://github.com/milenkovicm/testcontainers-redpanda-rs/actions/workflows/basic.yml/badge.svg)](https://github.com/milenkovicm/testcontainers-redpanda-rs/actions/workflows/basic.yml)
[![Crates.io](https://img.shields.io/crates/v/testcontainers-redpanda-rs)](https://crates.io/crates/testcontainers-redpanda-rs)
[![Crates.io](https://img.shields.io/crates/d/testcontainers-redpanda-rs)](https://crates.io/crates/testcontainers-redpanda-rs)

Unofficial testcontainer for [Redpanda](https://redpanda.com). Redpanda is a simple, powerful, and cost-efficient streaming data platform that is compatible with Kafka APIs but much less complex, faster and more affordable.

Add dependency:

```toml
testcontainers-redpanda-rs = { version = "0.13" }
```

Create and run redpanda container:

```rust, no_run
use testcontainers_redpanda_rs::*;

#[tokio::main]
async fn main() {
    let container = Redpanda::default();

    let server_node = container.start().await.unwrap();
    let bootstrap_servers = format!("localhost:{}", server_node.get_host_port_ipv4(REDPANDA_PORT).await.unwrap());
    // if topic has only one partition this part is optional
    // it will be automatically created when client connects
    server_node.exec(Redpanda::cmd_create_topic("test_topic", 3)).await.unwrap();

    println!("Redpanda server: {}", bootstrap_servers);
}
```

Explicit dependency on `testcontainers` is not needed.

Note about version compatibility:

- `0.13.x` supports `testcontainers` `0.26`
- `0.12.x` supports `testcontainers` `0.25`
- `0.11.x` supports `testcontainers` `0.24`
- `0.10.x` supports `testcontainers` `0.23`
