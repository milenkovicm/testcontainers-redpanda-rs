# Testcontainer(-rs) for Redpanda

Unofficial testcontainer for [Redpanda](https://redpanda.com).

Add dependency:

```toml
testcontainers-redpanda-rs = "0.1"
```

Usage example:

```rust
use testcontainers::clients;
use testcontainers_redpanda_rs::*;

let docker = clients::Cli::default();
let container = Redpanda::latest();

let redpanda_node = docker.run(container);
// auto create topic is enabled 
// use this to create topic with specific number
// of partitions.
redpanda_node.exec(Redpanda::cmd_create_topic("new_topic", 3));
let redpanda_server_address = format!("localhost:{}", redpanda_node.get_host_port_ipv4(REDPANDA_PORT));

println!("red panda server: {}", redpanda_server_address);
```

Current limitations:

* It will use default kafka ports and only one test can  at any time on given host. It was too complicated getting it right.
