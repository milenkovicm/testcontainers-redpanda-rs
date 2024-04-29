#![doc = include_str!("../README.md")]

use testcontainers::{
    core::{ContainerState, ExecCommand, WaitFor},
    Image, ImageArgs, RunnableImage,
};
/// Redpanda/Kafka API port
pub const REDPANDA_PORT: u16 = 9092;
/// Schema Registry Port
pub const SCHEMA_REGISTRY_PORT: u16 = 8081;
/// Prometheus and HTTP admin port
pub const ADMIN_PORT: u16 = 9644;

#[derive(Debug)]
pub struct Redpanda {
    tag: String,
}

impl Redpanda {
    /// creates test container for specified tag
    pub fn for_tag(tag: String) -> RunnableImage<Self> {
        RunnableImage::from(Self { tag })
            .with_mapped_port((REDPANDA_PORT, REDPANDA_PORT))
            .with_mapped_port((SCHEMA_REGISTRY_PORT, SCHEMA_REGISTRY_PORT))
            .with_mapped_port((ADMIN_PORT, ADMIN_PORT))
    }

    #[deprecated = "Use Self::latest()"]
    #[allow(clippy::should_implement_trait)]
    pub fn default() -> RunnableImage<Self> {
        Self::latest()
    }
    /// creates test container with `latest` tag
    pub fn latest() -> RunnableImage<Self> {
        Self::for_tag("latest".into())
    }
}

#[allow(dead_code)]
impl Redpanda {
    /// A command to create new topic with specified number of partitions
    ///
    /// # Arguments
    ///
    /// - `topic_name` name of the topic to be created
    /// - `partitions` number fo partitions for given topic
    pub fn cmd_create_topic(topic_name: &str, partitions: i32) -> ExecCommand {
        log::debug!("cmd create topic [{}], with [{}] partition(s)", topic_name, partitions);
        // not the best ready_condition
        let container_ready_conditions = vec![
            WaitFor::StdErrMessage {
                message: String::from("Create topics"),
            },
            WaitFor::Duration {
                length: std::time::Duration::from_secs(1),
            },
        ];

        //ExecCommand::new(vec![format!("rpk topic create {} -p {}", topic_name, partitions)])
        ExecCommand::new(vec![
            String::from("rpk"),
            String::from("topic"),
            String::from("create"),
            String::from(topic_name),
            String::from("-p"),
            format!("{}", partitions),
        ])
        .with_cmd_ready_condition(WaitFor::Duration {
            length: std::time::Duration::from_secs(1),
        })
        .with_container_ready_conditions(container_ready_conditions)
    }
}

#[derive(Debug, Default, Clone)]
pub struct RedpandaArgs {}

impl ImageArgs for RedpandaArgs {
    fn into_iterator(self) -> Box<dyn Iterator<Item = String>> {
        Box::new(
            vec![
                "-c".into(),
                "/usr/bin/rpk redpanda start --mode dev-container --node-id 0 --set redpanda.auto_create_topics_enabled=true"
                    .into(),
            ]
            .into_iter(),
        )
    }
}

// Test container should execute docker command similar to:
//
// ```
// docker run -ti --name=redpanda-1 --rm -p 9092:9092 -p 9644:9644 -p 8081:8081  docker.redpanda.com/redpandadata/redpanda redpanda start --mode dev-container --node-id 0 --set redpanda.auto_create_topics_enabled=true
// ```

impl Image for Redpanda {
    type Args = RedpandaArgs;

    fn name(&self) -> String {
        // TODO: make container name configurable
        "docker.redpanda.com/redpandadata/redpanda".into()
    }

    fn tag(&self) -> String {
        self.tag.to_owned()
    }

    fn ready_conditions(&self) -> Vec<testcontainers::core::WaitFor> {
        vec![
            WaitFor::StdErrMessage {
                // this is better message to wait for than
                // message: String::from("Successfully started Redpanda!"),
                // as at that point cluster will be initialized and client will retrieve
                // right cluster id.
                message: String::from("Initialized cluster_id to "),
            },
            // No need to wait for cluster to settle down if we get `Initialized cluster_id to` message
            // WaitFor::Duration {
            //     length: std::time::Duration::from_secs(1),
            // },
        ]
    }

    fn entrypoint(&self) -> Option<String> {
        Some("sh".into())
    }

    fn expose_ports(&self) -> Vec<u16> {
        // this is not needed as we map it explicitly
        // and testcontainer gets confused and re-map it
        // vec![REDPANDA_PORT, SCHEMA_REGISTRY_PORT, ADMIN_PORT]
        vec![]
    }

    fn exec_after_start(&self, _: ContainerState) -> Vec<ExecCommand> {
        vec![]
    }
}

#[cfg(test)]
#[ctor::ctor]
fn init() {
    // Enable RUST_LOG logging configuration for test
    let _ = env_logger::builder().is_test(true).try_init();
}
