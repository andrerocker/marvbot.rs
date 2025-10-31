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
    config::{Config, MarvSetup},
    metrics::MARV_PLUGIN_KAFKA_CONSUME_COUNTER,
    plugins::Plugin,
};

pub struct KafkaConsumer {}

impl KafkaConsumer {
    pub fn new(setup: &MarvSetup) -> Box<dyn Plugin> {
        let setup = setup.clone();

        thread::spawn(|| {
            handle_messages(setup).unwrap();
        });

        return Box::new(KafkaConsumer {});
    }
}

fn handle_messages(setup: MarvSetup) -> Result<(), KafkaError> {
    let config = setup.config.clone();
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
                save_message(config.clone(), message);
            }
            consumer.consume_messageset(ms).unwrap();
        }

        consumer
            .commit_consumed()
            .expect("Problems trying to commit consumed messages");
    }
}

fn save_message(config: Config, message: &Message) {
    //TODO: Dirty way to deal with files. It's just a Quick and Dirty Impl
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
        return false;
    }

    fn perform(&mut self, _message: &String) -> Result<Vec<String>, Error> {
        return Ok(vec![]);
    }
}
