#![doc = include_str!("../README.md")]

use testcontainers::{
    core::{ContainerState, ExecCommand, WaitFor},
    Image, ImageArgs, RunnableImage,
};

pub const REDPANDA_PORT: u16 = 9092;
pub const SCHEMA_REGISTRY_PORT: u16 = 8081;

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
    pub fn cmd_create_topic(topic_name: &str, partitions: i32) -> ExecCommand {
        log::debug!("cmd create topic [{}], with [{}] partition(s0", topic_name, partitions);
        let ready_conditions = vec![
            WaitFor::StdErrMessage {
                message: String::from("Create topics"),
            },
            WaitFor::Duration {
                length: std::time::Duration::from_secs(1),
            },
        ];

        ExecCommand {
            cmd: format!("rpk topic create {} -p {}", topic_name, partitions),
            ready_conditions,
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct RedpandaArgs {}

impl ImageArgs for RedpandaArgs {
    fn into_iterator(self) -> Box<dyn Iterator<Item = String>> {
        Box::new(
            vec![
                "-c".into(),
                "/usr/bin/rpk redpanda start --check=false --node-id 0 --set redpanda.auto_create_topics_enabled=true"
                    .into(),
            ]
            .into_iter(),
        )
    }
}

impl Image for Redpanda {
    type Args = RedpandaArgs;

    fn name(&self) -> String {
        "docker.redpanda.com/redpandadata/redpanda".into()
        // there is a change in container name (and location)
        // "docker.vectorized.io/vectorized/redpanda".into()
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
        vec![REDPANDA_PORT, SCHEMA_REGISTRY_PORT]
    }

    fn exec_after_start(&self, cs: ContainerState) -> Vec<ExecCommand> {
        log::info!(
            "setting extra configuration for test container ... port: {}",
            cs.host_port_ipv4(REDPANDA_PORT)
        );

        vec![]
    }
}
