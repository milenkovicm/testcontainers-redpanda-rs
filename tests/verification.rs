mod common;
#[cfg(test)]
mod test {
    use crate::common::*;
    use testcontainers_redpanda_rs::*;

    #[tokio::test]
    #[serial_test::serial]
    async fn should_start_redpanda_server_send_messages() {
        let container = Redpanda::latest();

        let instance = container.start().await;
        let bootstrap_servers = format!("localhost:{}", instance.get_host_port_ipv4(REDPANDA_PORT).await);
        log::info!("bootstrap servers: {}", bootstrap_servers);

        let test_topic_name = random_topic_name();
        log::info!("populating topic: [{}] ...", test_topic_name);
        populate_topic(&bootstrap_servers, &test_topic_name, 10, &value_fn, &key_fn, None, None).await;
    }

    #[tokio::test]
    #[serial_test::serial]
    async fn should_start_redpanda_server_crate_topic_send_messages_to_partition() {
        let container = Redpanda::latest();

        let instance = container.start().await;
        let bootstrap_servers = format!("localhost:{}", instance.get_host_port_ipv4(REDPANDA_PORT).await);

        // if topic has only one partition this part is optional
        // it will be automatically created when client connects
        let test_topic_name = &random_topic_name();
        log::info!("creating topic: [{}] ...", test_topic_name);
        instance.exec(Redpanda::cmd_create_topic(test_topic_name, 3)).await;

        log::info!("bootstrap servers: {}", bootstrap_servers);

        log::info!("populating topic: [{}] ...", test_topic_name);
        populate_topic(
            &bootstrap_servers,
            &test_topic_name,
            10,
            &value_fn,
            &key_fn,
            Some(2),
            None,
        )
        .await;
    }

    #[tokio::test]
    #[serial_test::serial]
    async fn should_expose_admin_api() {
        let container = Redpanda::latest();

        let instance = container.start().await;
        let address_admin_api = format!("http://localhost:{}/v1", instance.get_host_port_ipv4(ADMIN_PORT).await);

        let response = reqwest::get(address_admin_api).await.expect("admin http response");

        assert_eq!(200, response.status().as_u16());
    }

    #[tokio::test]
    #[serial_test::serial]
    async fn should_expose_schema_registry_api() {
        let container = Redpanda::latest();

        let instance = container.start().await;
        let address_schema_registry = format!(
            "http://localhost:{}/v1",
            instance.get_host_port_ipv4(SCHEMA_REGISTRY_PORT).await
        );

        let response = reqwest::get(address_schema_registry)
            .await
            .expect("admin http response");

        assert_eq!(200, response.status().as_u16());
    }
}
