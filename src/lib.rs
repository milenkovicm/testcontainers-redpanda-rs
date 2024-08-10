#![doc = include_str!("../README.md")]

use testcontainers::{
    core::{wait::LogWaitStrategy, CmdWaitFor, ContainerPort, ContainerState, ExecCommand, WaitFor},
    ContainerRequest, Image, TestcontainersError,
};

pub use testcontainers::runners::AsyncRunner;
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
    pub fn for_tag(tag: String) -> ContainerRequest<Self> {
        ContainerRequest::from(Self { tag })
        //.with_mapped_port(REDPANDA_PORT, ContainerPort::Tcp(REDPANDA_PORT))
        //.with_mapped_port(SCHEMA_REGISTRY_PORT, ContainerPort::Tcp(SCHEMA_REGISTRY_PORT))
        //.with_mapped_port(ADMIN_PORT, ContainerPort::Tcp(ADMIN_PORT))
    }

    #[deprecated = "Use Self::latest()"]
    #[allow(clippy::should_implement_trait)]
    pub fn default() -> ContainerRequest<Self> {
        Self::latest()
    }
    /// creates test container with `latest` tag
    pub fn latest() -> ContainerRequest<Self> {
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
            WaitFor::Log(LogWaitStrategy::stderr("Create topics")),
            WaitFor::Duration {
                length: std::time::Duration::from_secs(1),
            },
        ];

        // ExecCommand::new(vec![format!("rpk topic create {} -p {}", topic_name, partitions)])
        ExecCommand::new(vec![
            String::from("rpk"),
            String::from("topic"),
            String::from("create"),
            String::from(topic_name),
            String::from("-p"),
            partitions.to_string(),
        ])
        .with_cmd_ready_condition(CmdWaitFor::Duration {
            length: std::time::Duration::from_secs(1),
        })
        .with_container_ready_conditions(container_ready_conditions)
    }
}

// Test container should execute docker command similar to:
//
// ```
// docker run -ti --name=redpanda-1 --rm -p 9092:9092 -p 9644:9644 -p 8081:8081  docker.redpanda.com/redpandadata/redpanda redpanda start --mode dev-container --node-id 0 --set redpanda.auto_create_topics_enabled=true
// ```
//

impl Image for Redpanda {
    fn name(&self) -> &str {
        "docker.redpanda.com/redpandadata/redpanda"
    }

    //withCommand("sh -c while [ ! -f /start-panda.sh  ]; do sleep 0.1; done; /start-panda.sh);
    fn entrypoint(&self) -> Option<&str> {
        Some("sh")
    }

    fn cmd(&self) -> impl IntoIterator<Item = impl Into<std::borrow::Cow<'_, str>>> {
        vec![
            "-c",
            "while [ ! -f /tmp/start-panda.sh  ]; do sleep 0.1; done; sh /tmp/start-panda.sh",
        ]
        .into_iter()
    }

    fn tag(&self) -> &str {
        self.tag.as_str()
    }

    fn ready_conditions(&self) -> Vec<testcontainers::core::WaitFor> {
        vec![
            // this has been moved to exec_after_start 
            //
            // WaitFor::Log(LogWaitStrategy::stderr("Initialized cluster_id to ")),
            // No need to wait for cluster to settle down if we get `Initialized cluster_id to` message
            // WaitFor::Duration {
            //     length: std::time::Duration::from_secs(1),
            // },
        ]
    }

    fn expose_ports(&self) -> &[ContainerPort] {
        &[]
    }

    fn exec_after_start(&self, state: ContainerState) -> Result<Vec<ExecCommand>, TestcontainersError> {
        let c = ExecCommand::new(vec![
            "sh", "-c",
            format!("echo '/usr/bin/rpk redpanda start --mode dev-container  --node-id 0 --set redpanda.auto_create_topics_enabled=true --kafka-addr INTERNAL://0.0.0.0:29092,EXTERNAL://0.0.0.0:9092 --advertise-kafka-addr INTERNAL://localhost:29092,EXTERNAL://localhost:{}' > /tmp/start-panda.sh", state.host_port_ipv4(ContainerPort::Tcp(REDPANDA_PORT)).unwrap()).as_str()
        ]).with_container_ready_conditions(
            vec![
                WaitFor::Log(LogWaitStrategy::stderr("Initialized cluster_id to "))
            ]
        );

        Ok(vec![c])
    }
}

#[cfg(test)]
#[ctor::ctor]
fn init() {
    // Enable RUST_LOG logging configuration for test
    let _ = env_logger::builder().is_test(true).try_init();
}
