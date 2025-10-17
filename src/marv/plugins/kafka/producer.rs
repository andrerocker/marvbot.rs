use crate::marv::{config::MarvSetup, plugins::Plugin};
use kafka::{
    client::RequiredAcks,
    producer::{Producer, Record},
};

pub struct KafkaProducer {
    pub topic: String,
    pub producer: Producer,
}

impl KafkaProducer {
    pub fn new(setup: &MarvSetup) -> Box<dyn Plugin> {
        let brokers = vec![setup.config.broker.to_string()];
        let producer = Producer::from_hosts(brokers)
            .with_required_acks(RequiredAcks::One)
            .create()
            .unwrap();

        Box::new(KafkaProducer {
            topic: setup.config.topic.to_string(),
            producer: producer,
        })
    }
}

impl Plugin for KafkaProducer {
    fn is_enabled(&self, _message: &String) -> bool {
        return true;
    }

    fn perform(&mut self, message: &String) -> Vec<String> {
        self.producer
            .send(&Record::from_value(&self.topic, message.as_bytes()))
            .unwrap();

        return vec![];
    }
}
