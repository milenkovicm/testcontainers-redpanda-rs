mod utils;
#[cfg(test)]
mod test {

    use crate::utils::*;
    use testcontainers::clients;
    use testcontainers_redpanda_rs::*;

    #[tokio::test]
    async fn should_start_redpanda_server_send_messages() {
        setup_logger(true, Some("testcontainers=debug,testcontainer_addons=debug"));
        let docker = clients::Cli::default();
        let container = Redpanda::default();

        let server_node = docker.run(container);
        let bootstrap_servers = format!("localhost:{}", server_node.get_host_port_ipv4(REDPANDA_PORT));

        println!("bootstrap servers: {}", bootstrap_servers);
        std::env::set_var("KAFKA_HOST", &bootstrap_servers);

        assert!(bootstrap_servers.len() > 10);
        let test_topic_name = "test_topic";

        log::info!("populating topic: [{}] ...", test_topic_name);
        populate_topic(&test_topic_name, 10, &value_fn, &key_fn, Some(0), None).await;
    }

    #[tokio::test]
    async fn should_start_redpanda_server_crate_topic_send_messages_to_partition() {
        setup_logger(true, Some("testcontainers=debug,testcontainer_addons=debug"));

        let docker = clients::Cli::default();
        let container = Redpanda::default();

        let server_node = docker.run(container);
        let bootstrap_servers = format!("localhost:{}", server_node.get_host_port_ipv4(REDPANDA_PORT));

        let test_topic_name = "test_topic";
        server_node.exec(Redpanda::cmd_create_topic(test_topic_name, 3));

        println!("bootstrap servers: {}", bootstrap_servers);
        std::env::set_var("KAFKA_HOST", &bootstrap_servers);

        assert!(bootstrap_servers.len() > 10);

        log::info!("populating topic: [{}] ...", test_topic_name);
        populate_topic(&test_topic_name, 10, &value_fn, &key_fn, Some(2), None).await;
    }
}
