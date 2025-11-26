use async_trait::async_trait;
use marv_api::{
    config,
    plugins::{DynamicPlugin, Plugin},
};
use rdkafka::{
    ClientConfig,
    message::ToBytes,
    producer::{FutureProducer, FutureRecord},
};
use std::{io::Error, time::Duration};

pub struct KafkaProducer {
    pub topic: String,
    pub producer: FutureProducer,
}

impl KafkaProducer {
    pub fn new() -> DynamicPlugin {
        let config = config::config();

        let producer: FutureProducer = ClientConfig::new()
            .set("bootstrap.servers", "localhost:9092")
            .set("message.timeout.ms", "5000")
            .create()
            .expect("Producer creation error");

        Box::new(KafkaProducer {
            topic: config.topic.clone(),
            producer: producer,
        })
    }
}

#[async_trait]
impl Plugin for KafkaProducer {
    fn name(&self) -> String {
        "KafkaProducer".into()
    }

    fn responds_to(&self, _message: &String) -> bool {
        true
    }

    fn schedule(&self) -> Option<String> {
        None
    }

    async fn perform(&self, message: &String) -> Result<Vec<String>, Error> {
        let serialized_message = serde_cbor::to_vec(message).unwrap();

        let produce_message = self.producer.send(
            FutureRecord::to(&self.topic)
                .payload(serialized_message.to_bytes())
                .key("a-key"),
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
