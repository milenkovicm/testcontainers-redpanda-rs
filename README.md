# Unofficial Rust Test Container For Redpanda

[![github action](https://github.com/milenkovicm/testcontainers-redpanda-rs/actions/workflows/basic.yml/badge.svg)](https://github.com/milenkovicm/testcontainers-redpanda-rs/actions/workflows/basic.yml)
[![Crates.io](https://img.shields.io/crates/v/testcontainers-redpanda-rs)](https://crates.io/crates/testcontainers-redpanda-rs)
[![Crates.io](https://img.shields.io/crates/d/testcontainers-redpanda-rs)](https://crates.io/crates/testcontainers-redpanda-rs)

Unofficial testcontainer for [Redpanda](https://redpanda.com).

Add dependency:

```toml
testcontainers-redpanda-rs = "0.1"
```

Create and run redpanda container:

```rust, no_run
use testcontainers::clients;
use testcontainers_redpanda_rs::*;

let container = Redpanda::latest();

let server_node = container.start().await;
let bootstrap_servers = format!("localhost:{}", server_node.get_host_port_ipv4(REDPANDA_PORT).await);
// if topic has only one partition this part is optional
// it will be automatically created when client connects
let test_topic_name = "test_topic";
server_node.exec(Redpanda::cmd_create_topic(test_topic_name, 3)).await;

println!("Redpanda server: {}", bootstrap_servers);
```

Current limitations:

* It will use default kafka ports and only one test can  at any time on given host. It was too complicated getting it right.
