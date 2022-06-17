# Testcontainer(-rs) for Redpanda

```rust
use testcontainers::clients;
use testcontainers_redpanda_rs::*;

let docker = clients::Cli::default();
let container = Redpanda::default();

let server_node = docker.run(container);
let bootstrap_servers = format!("localhost:{}", server_node.get_host_port_ipv4(REDPANDA_PORT));

println!("bootstrap servers: {}", bootstrap_servers);
```

[Have a look at tests](tests/redpanda.rs) for more usage examples.
