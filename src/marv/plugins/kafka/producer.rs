use std::{io::Error, time::Duration};

use crate::marv::plugins::{DynamicPlugin, Plugin};
use async_trait::async_trait;
use rdkafka::{
    ClientConfig,
    producer::{FutureProducer, FutureRecord},
};

pub struct KafkaProducer {
    pub topic: String,
    pub producer: FutureProducer,
}

impl KafkaProducer {
    pub fn new() -> DynamicPlugin {
        let producer: FutureProducer = ClientConfig::new()
            .set("bootstrap.servers", "localhost:9092")
            .set("message.timeout.ms", "5000")
            .create()
            .expect("Producer creation error");

        Box::new(KafkaProducer {
            topic: "Bacon".to_string(),
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
        let produce_message = self.producer.send(
            FutureRecord::to(&self.topic).payload(&message).key("a-key"),
            Duration::from_secs(0),
        );

        if let Err((error, _msg)) = produce_message.await {
            log::error!(
                "Problems trying to produce message to the broker: {}",
                error
            );
        }

        Ok(vec![])
    }
}
