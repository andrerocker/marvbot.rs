use std::io::Error;

use crate::marv::{
    config::{self},
    plugins::Plugin,
};
use kafka::{
    client::RequiredAcks,
    producer::{Producer, Record},
};

pub struct KafkaProducer {
    pub topic: String,
    pub producer: Producer,
}

impl KafkaProducer {
    pub fn new() -> Box<dyn Plugin> {
        let config = &config::CONFIG.config;

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

impl Plugin for KafkaProducer {
    fn name(&self) -> String {
        "KafkaProducer".to_string()
    }

    fn is_enabled(&self, _message: &String) -> bool {
        true
    }

    fn perform(&mut self, message: &String) -> Result<Vec<String>, Error> {
        let record = &Record::from_value(&self.topic, message.as_bytes());
        self.producer
            .send(record)
            .expect("Problems trying to write message");

        Ok(vec![])
    }
}
