mod common;
#[cfg(test)]
mod test {
    use crate::common::*;
    use log::info;
    use testcontainers::runners::AsyncRunner;
    use testcontainers_redpanda_rs::*;

    #[tokio::test]
    async fn should_start_redpanda_server_send_messages() {
        let container = Redpanda::latest();

        let server_node = container
            //.with_mapped_port(PortMapping {local: REDPANDA_PORT, internal: REDPANDA_PORT})
            .start()
            .await;

        let bootstrap_servers = format!(
            "{}:{}",
            "localhost",
            server_node.get_host_port_ipv4(REDPANDA_PORT).await
        );

        info!("bootstrap servers: {}", bootstrap_servers);
        std::env::set_var("KAFKA_HOST", &bootstrap_servers);

        assert!(bootstrap_servers.len() > 10);
        let test_topic_name = "test_topic";

        log::info!("populating topic: [{}] ...", test_topic_name);
        populate_topic(&test_topic_name, 10, &value_fn, &key_fn, Some(0), None).await;
    }
}
