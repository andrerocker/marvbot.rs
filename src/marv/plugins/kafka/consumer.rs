use std::{
    fs::OpenOptions,
    io::{Error, Write},
    thread,
};

use kafka::{
    client::{FetchOffset, GroupOffsetStorage},
    consumer::{Consumer, Message},
    error::Error as KafkaError,
};

use crate::marv::{
    config::{self},
    metrics::MARV_PLUGIN_KAFKA_CONSUME_COUNTER,
    plugins::{DynamicPlugin, Plugin},
};

pub struct KafkaConsumer {}

impl KafkaConsumer {
    pub fn new() -> DynamicPlugin {
        thread::spawn(|| {
            handle_messages().unwrap();
        });

        Box::new(KafkaConsumer {})
    }
}

fn handle_messages() -> Result<(), KafkaError> {
    let config = &config::CONFIG.config;
    let topic = config.topic.clone();
    let group = config.group.clone();
    let brokers = vec![config.broker.clone()];

    let mut consumer = Consumer::from_hosts(brokers)
        .with_topic(topic)
        .with_group(group)
        .with_fallback_offset(FetchOffset::Earliest)
        .with_offset_storage(Some(GroupOffsetStorage::Kafka))
        .create()
        .expect("Problems trying to initialize Consumer");

    loop {
        for ms in consumer.poll().unwrap().iter() {
            for message in ms.messages() {
                MARV_PLUGIN_KAFKA_CONSUME_COUNTER.inc();
                save_message(message);
            }
            consumer.consume_messageset(ms).unwrap();
        }

        consumer
            .commit_consumed()
            .expect("Problems trying to commit consumed messages");
    }
}

fn save_message(message: &Message) {
    //TODO: Dirty way to deal with files. It's just a Quick and Dirty Impl
    let config = &config::CONFIG.config;
    let target_file = config.messages_log.clone();
    let contents = message.value; // std::str::from_utf8(message.value).unwrap();
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(target_file)
        .expect("Problems trying to open the messages file");

    file.write_all(contents)
        .expect("Problems trying to write to the messages file")
}

impl Plugin for KafkaConsumer {
    fn name(&self) -> String {
        "KafkaConsumer".to_string()
    }

    fn is_enabled(&self, _message: &String) -> bool {
        false
    }

    fn perform(&mut self, _message: &String) -> Result<Vec<String>, Error> {
        Ok(vec![])
    }
}
