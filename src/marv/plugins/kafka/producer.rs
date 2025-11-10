use std::io::Error;

use crate::marv::{
    config::{self},
    plugins::{DynamicPlugin, Plugin},
};
use async_trait::async_trait;
use kafka::{
    client::RequiredAcks,
    producer::{Producer, Record},
};

pub struct KafkaProducer {
    pub topic: String,
    pub producer: Producer,
}

impl KafkaProducer {
    pub fn new() -> DynamicPlugin {
        let config = &config::MARV.config;

        let brokers = vec![config.broker.to_string()];
        let producer = Producer::from_hosts(brokers)
            .with_required_acks(RequiredAcks::One)
            .create()
            .expect("Problems trying to initialize Producer");

        Box::new(KafkaProducer {
            topic: config.topic.to_string(),
            producer: producer,
        })
    }
}

#[async_trait]
impl Plugin for KafkaProducer {
    fn name(&self) -> String {
        "KafkaProducer".to_string()
    }

    async fn is_enabled(&self, _message: &String) -> bool {
        true
    }

    async fn perform(&mut self, message: &String) -> Result<Vec<String>, Error> {
        let record = &Record::from_value(&self.topic, message.as_bytes());
        self.producer
            .send(record)
            .expect("Problems trying to write message");

        Ok(vec![])
    }
}
