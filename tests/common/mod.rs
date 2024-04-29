//!
//! # Common For tests
//!

#![allow(dead_code)]

use rdkafka::admin::{AdminClient, AdminOptions, NewTopic, TopicReplication};
use rdkafka::client::ClientContext;
use rdkafka::config::ClientConfig;
use rdkafka::consumer::ConsumerContext;
use rdkafka::error::KafkaResult;
use rdkafka::message::ToBytes;
use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::statistics::Statistics;
use rdkafka::TopicPartitionList;
use std::collections::HashMap;
use std::env;

use std::time::Duration;

/// Taken from https://github.com/fede1024/rust-rdkafka/blob/master/tests/utils.rs with some slight modifications and updates
/// credit to rdkafka

pub fn rand_test_topic() -> String {
    format!("__test_{}", random_topic_name())
}

pub fn rand_test_group() -> String {
    format!("__test_{}", random_topic_name())
}

pub fn rand_test_transactional_id() -> String {
    format!("__test_{}", random_topic_name())
}

pub fn get_bootstrap_server() -> String {
    env::var("KAFKA_HOST").unwrap_or_else(|_| "localhost:9092".to_owned())
}

pub struct ProducerTestContext {
    _some_data: i64,
}

impl ClientContext for ProducerTestContext {
    fn stats(&self, _: Statistics) {} // Don't print stats
}

pub async fn create_topic(name: &str, partitions: i32) {
    let client: AdminClient<_> = consumer_config("create_topic", None)
        .into_iter()
        .collect::<ClientConfig>()
        .create()
        .unwrap();
    client
        .create_topics(
            &[NewTopic::new(name, partitions, TopicReplication::Fixed(1))],
            &AdminOptions::new(),
        )
        .await
        .unwrap();
}

/// Produce the specified count of messages to the topic and partition specified. A map
/// of (partition, offset) -> message id will be returned. It panics if any error is encountered
/// while populating the topic.
pub async fn populate_topic<P, K, J, Q>(
    topic_name: &str,
    count: i32,
    value_fn: &P,
    key_fn: &K,
    partition: Option<i32>,
    timestamp: Option<i64>,
) -> HashMap<(i32, i64), i32>
where
    P: Fn(i32) -> J,
    K: Fn(i32) -> Q,
    J: ToBytes,
    Q: ToBytes,
{
    let prod_context = ProducerTestContext { _some_data: 1234 };

    // Produce some messages
    let producer = &ClientConfig::new()
        .set("bootstrap.servers", get_bootstrap_server().as_str())
        .set("statistics.interval.ms", "500")
        .set("api.version.request", "true")
        .set("debug", "all")
        .set("message.timeout.ms", "10000")
        .create_with_context::<ProducerTestContext, FutureProducer<_>>(prod_context)
        .expect("Producer creation error");

    let futures = (0..count)
        .map(|id| {
            let future = async move {
                producer
                    .send(
                        FutureRecord {
                            topic: topic_name,
                            payload: Some(&value_fn(id)),
                            key: Some(&key_fn(id)),
                            partition,
                            timestamp,
                            headers: None,
                        },
                        Duration::from_secs(1),
                    )
                    .await
            };
            (id, future)
        })
        .collect::<Vec<_>>();

    let mut message_map = HashMap::new();
    for (id, future) in futures {
        match future.await {
            Ok((partition, offset)) => message_map.insert((partition, offset), id),
            Err((kafka_error, _message)) => panic!("Delivery failed: {}", kafka_error),
        };
    }

    message_map
}

pub fn value_fn(id: i32) -> String {
    format!("Message {}", id)
}

pub fn key_fn(id: i32) -> String {
    format!("Key {}", id)
}

pub fn random_topic_name() -> String {
    rusty_ulid::generate_ulid_string()
}

pub struct ConsumerTestContext {
    pub _n: i64, // Add data for memory access validation
}

impl ClientContext for ConsumerTestContext {
    // Access stats
    fn stats(&self, stats: Statistics) {
        let stats_str = format!("{:?}", stats);
        log::info!("Stats received: {} bytes", stats_str.len());
    }
}

impl ConsumerContext for ConsumerTestContext {
    fn commit_callback(&self, result: KafkaResult<()>, _offsets: &TopicPartitionList) {
        log::info!("Committing offsets: {:?}", result);
    }
}

pub fn consumer_config<'a>(
    group_id: &'a str,
    config_overrides: Option<HashMap<&'a str, &'a str>>,
) -> HashMap<String, String> {
    let mut config: HashMap<String, String> = HashMap::new();

    config.insert("group.id".into(), group_id.into());
    config.insert("client.id".into(), "datafusion-streams".into());
    config.insert("bootstrap.servers".into(), get_bootstrap_server());
    config.insert("enable.partition.eof".into(), "true".into());
    config.insert("session.timeout.ms".into(), "6000".into());
    config.insert("enable.auto.commit".into(), "true".into());
    config.insert("statistics.interval.ms".into(), "500".into());
    config.insert("api.version.request".into(), "true".into());
    config.insert("debug".into(), "all".into());
    config.insert("auto.offset.reset".into(), "earliest".into());

    if let Some(overrides) = config_overrides {
        for (key, value) in overrides {
            config.insert(key.into(), value.into());
        }
    }

    config
}

#[cfg(test)]
#[ctor::ctor]
fn init() {
    // Enable RUST_LOG logging configuration for test
    let _ = env_logger::builder().is_test(true).try_init();
}
