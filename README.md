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

let docker = clients::Cli::default();
let container = Redpanda::latest();
let redpanda_node = docker.run(container);

// Auto create topic is enabled by default,
// use this if you need to create a topic with 
// specific number of partitions. 
redpanda_node.exec(Redpanda::cmd_create_topic("new_topic", 3));

let redpanda_server_address = format!("localhost:{}", redpanda_node.get_host_port_ipv4(REDPANDA_PORT));

println!("Redpanda server: {}", redpanda_server_address);
```

Current limitations:

* It will use default kafka ports and only one test can  at any time on given host. It was too complicated getting it right.
