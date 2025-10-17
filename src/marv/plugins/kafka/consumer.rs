use std::thread;

use kafka::{
    client::{FetchOffset, GroupOffsetStorage},
    consumer::Consumer,
};

use kafka::error::Error as KafkaError;

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
    fn is_enabled(&self, _message: &String) -> bool {
        return false;
    }

    fn perform(&mut self, _message: &String) -> Vec<String> {
        return vec![];
    }
}

fn handle_messages(group: String, topic: String, brokers: Vec<String>) -> Result<(), KafkaError> {
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
                println!(
                    "Offset: {}, Key: {:?}, Value: {:?}",
                    message.offset, message.key, message.value
                );
            }
            consumer.consume_messageset(ms).unwrap();
        }

        consumer.commit_consumed().unwrap();
    }
}
