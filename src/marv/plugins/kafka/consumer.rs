use crate::marv::{
    config::{self},
    plugins::{DynamicPlugin, Plugin},
};
use async_trait::async_trait;
use rdkafka::{
    ClientConfig, Message,
    consumer::{CommitMode, Consumer, StreamConsumer},
};
use std::io::Error;

pub struct KafkaConsumer {}

impl KafkaConsumer {
    pub fn new() -> DynamicPlugin {
        tokio::task::spawn(async {
            handle_messages().await;
        });

        Box::new(KafkaConsumer {})
    }
}

#[async_trait]
impl Plugin for KafkaConsumer {
    fn name(&self) -> String {
        "KafkaConsumer".to_string()
    }

    async fn is_enabled(&self, _message: &String) -> bool {
        false
    }

    async fn perform(&mut self, _message: &String) -> Result<Vec<String>, Error> {
        Ok(vec![])
    }
}

async fn handle_messages() {
    let config = &config::MARV.config;
    let topic = config.topic.clone();
    let group = config.group.clone();
    let brokers = config.broker.clone();

    let consumer: StreamConsumer = ClientConfig::new()
        .set("bootstrap.servers", brokers)
        .set("group.id", group)
        .create()
        .expect("Consumer creation failed");

    consumer.subscribe(&[topic.as_str()]).unwrap();

    loop {
        match consumer.recv().await {
            Ok(msg) => {
                let payload = msg
                    .payload_view()
                    .map(|res| res.unwrap_or("<invalid utf-8>"))
                    .unwrap_or("<no payload>");

                log::info!("+++>> {}: {}", msg.offset(), payload);

                consumer.commit_message(&msg, CommitMode::Async).unwrap();
            }
            Err(e) => log::error!("Kafka error: {e}"),
        }
    }
}
