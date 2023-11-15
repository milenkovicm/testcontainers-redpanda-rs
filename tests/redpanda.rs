mod common;
#[cfg(test)]
mod test {
    use crate::common::*;
    use log::info;
    use testcontainers::clients;
    use testcontainers_redpanda_rs::*;

    #[tokio::test]
    async fn should_start_redpanda_server_send_messages() {
        setup_logger(
            true,
            Some("testcontainers=debug,testcontainer_addons=debug,testcontainers_redpanda_rs=debug"),
        );
        let docker = clients::Cli::default();
        let container = Redpanda::latest();

        let server_node = docker.run(container);
        let bootstrap_servers = format!("localhost:{}", server_node.get_host_port_ipv4(REDPANDA_PORT));

        info!("bootstrap servers: {}", bootstrap_servers);
        std::env::set_var("KAFKA_HOST", &bootstrap_servers);

        assert!(bootstrap_servers.len() > 10);
        let test_topic_name = "test_topic";

        log::info!("populating topic: [{}] ...", test_topic_name);
        populate_topic(&test_topic_name, 10, &value_fn, &key_fn, Some(0), None).await;
    }
}
