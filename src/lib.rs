use testcontainers::{
    core::{ContainerState, ExecCommand, WaitFor},
    Image, ImageArgs,
};

pub const REDPANDA_PORT: u16 = 9092;
pub const SCHEMA_REGISTRY_PORT: u16 = 8081;

#[derive(Default)]
pub struct Redpanda {}

#[allow(dead_code)]
impl Redpanda {
    pub fn cmd_create_topic(topic_name: &str, partitions: i32) -> ExecCommand {
        log::debug!("cmd create topic [{}], with [{}] partition(s0", topic_name, partitions);
        let ready_conditions = vec![WaitFor::Duration {
            length: std::time::Duration::from_millis(100),
        }];

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
                "while true; do echo \"*** container started ***\" ; sleep infinity; done".into(),
            ]
            .into_iter(),
        )
    }
}

impl Image for Redpanda {
    type Args = RedpandaArgs;

    fn name(&self) -> String {
        // we keep my version of redpanda as we know config options
        // are correct.
        // "milenkovicm/testcontainer-redpanda".into()
        "docker.vectorized.io/vectorized/redpanda".into()
    }

    fn tag(&self) -> String {
        // "latest".into()
        "v22.1.3".into()
        //"v21.11.15".into()
    }

    fn ready_conditions(&self) -> Vec<testcontainers::core::WaitFor> {
        vec![WaitFor::StdOutMessage {
            message: String::from("*** container started ***"),
        }]
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

        let mut commands = vec![];

        let cmd0 = format!(
            "/usr/bin/rpk redpanda start --check=false --node-id 0 --set redpanda.auto_create_topics_enabled=true --kafka-addr PLAINTEXT://0.0.0.0:29092,OUTSIDE://0.0.0.0:9092 --advertise-kafka-addr PLAINTEXT://localhost:29092,OUTSIDE://localhost:{}",
            cs.host_port_ipv4(REDPANDA_PORT)
        );

        let ready_wait_conditions = vec![WaitFor::Duration {
            length: std::time::Duration::from_secs(2),
        }];

        commands.push(ExecCommand {
            cmd: cmd0.into(),
            ready_conditions: ready_wait_conditions,
        });

        commands
    }
}
