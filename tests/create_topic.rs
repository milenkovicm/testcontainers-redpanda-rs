mod common;

#[cfg(test)]
mod test {
    use crate::common::*;
    use log::info;
    use testcontainers_redpanda_rs::*;

    #[tokio::test]
    async fn should_start_redpanda_server_crate_topic_send_messages_to_partition() {
        let container = Redpanda::latest();

        let server_node = container.start().await;
        let bootstrap_servers = format!("localhost:{}", server_node.get_host_port_ipv4(REDPANDA_PORT).await);

        // if topic has only one partition this part is optional
        // it will be automatically created when client connects
        let test_topic_name = &random_topic_name();
        server_node.exec(Redpanda::cmd_create_topic(test_topic_name, 3)).await;

        info!("bootstrap servers: {}", bootstrap_servers);
        std::env::set_var("KAFKA_HOST", &bootstrap_servers);

        log::info!("populating topic: [{}] ...", test_topic_name);
        populate_topic(&test_topic_name, 10, &value_fn, &key_fn, Some(2), None).await;
    }
}
