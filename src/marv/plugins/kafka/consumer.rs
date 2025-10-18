use std::thread;

use kafka::{
    client::{FetchOffset, GroupOffsetStorage},
    consumer::Consumer,
};

use kafka::error::Error as KafkaError;
use log::info;
use prometheus_exporter::prometheus::{register_counter, register_counter_vec};

use crate::marv::{config::MarvSetup, plugins::Plugin};

pub struct KafkaConsumer {
    // pub topic: String,
    // pub consumer: Consumer,
}

impl KafkaConsumer {
    pub fn new(setup: &MarvSetup) -> Box<dyn Plugin> {
        let config = setup.config.clone();
        let topic = config.topic.clone();
        let group = config.group.clone();
        let brokers = vec![setup.config.broker.to_string()];

        thread::spawn(|| {
            handle_messages(group, topic, brokers).unwrap();
        });

        return Box::new(KafkaConsumer {});
    }
}

impl Plugin for KafkaConsumer {
    fn name(&self) -> String {
        return "KafkaConsumer".to_string();
    }

    fn is_enabled(&self, _message: &String) -> bool {
        return false;
    }

    fn perform(&mut self, _message: &String) -> Vec<String> {
        return vec![];
    }
}

fn handle_messages(group: String, topic: String, brokers: Vec<String>) -> Result<(), KafkaError> {
    let consume_counter = register_counter!(
        "marv_plugin_kafka_consumer_consume_counter",
        "Track how many messages was consumed from Kafka Consumer",
    )
    .unwrap();

    let mut consumer = Consumer::from_hosts(brokers)
        .with_topic(topic.clone())
        .with_group(group.clone())
        .with_fallback_offset(FetchOffset::Earliest)
        .with_offset_storage(Some(GroupOffsetStorage::Kafka))
        .create()
        .unwrap();

    loop {
        for ms in consumer.poll().unwrap().iter() {
            for message in ms.messages() {
                consume_counter.inc();
                info!(
                    "Offset: {}, Key: {:?}, Value: {:?}",
                    message.offset,
                    message.key,
                    message.value.len()
                );
            }
            consumer.consume_messageset(ms).unwrap();
        }

        consumer.commit_consumed().unwrap();
    }
}
