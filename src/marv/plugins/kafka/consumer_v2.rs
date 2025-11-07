use crate::marv::{
    config::{self},
    plugins::{DynamicPlugin, Plugin},
};
use rdkafka::{
    ClientConfig, Message,
    consumer::{CommitMode, Consumer, StreamConsumer},
};
use std::{io::Error, thread};

pub struct KafkaV2Consumer {}

impl KafkaV2Consumer {
    pub fn new() -> DynamicPlugin {
        thread::spawn(|| handle_messages_v2());

        Box::new(KafkaV2Consumer {})
    }
}

impl Plugin for KafkaV2Consumer {
    fn name(&self) -> String {
        "KafkaV2Consumer".to_string()
    }

    fn is_enabled(&self, _message: &String) -> bool {
        false
    }

    fn perform(&mut self, _message: &String) -> Result<Vec<String>, Error> {
        Ok(vec![])
    }
}

#[tokio::main]
async fn handle_messages_v2() {
    let config = &config::CONFIG.config;
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

                log::info!(
                    "=------------------>> message @{}: {}",
                    msg.offset(),
                    payload
                );

                consumer.commit_message(&msg, CommitMode::Async).unwrap();
            }
            Err(e) => log::error!("Kafka error: {e}"),
        }
    }
}
