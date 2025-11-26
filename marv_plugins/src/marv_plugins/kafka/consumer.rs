use async_trait::async_trait;
use marv_api::{
    config, helper,
    plugins::{DynamicPlugin, Plugin},
};
use rdkafka::{
    ClientConfig, Message,
    consumer::{CommitMode, Consumer, StreamConsumer},
    message::BorrowedMessage,
};
use std::io::Error;

pub struct KafkaConsumer {}

impl KafkaConsumer {
    pub fn new() -> DynamicPlugin {
        tokio::task::spawn(async {
            match attach_and_handle().await {
                Ok(_) => log::info!("Stopping Marvbot!!!"),
                Err(error) => log::error!(
                    "Something wrong happen with Kafka message Consumer: {} ",
                    error
                ),
            }
        });

        Box::new(KafkaConsumer {})
    }
}

#[async_trait]
impl Plugin for KafkaConsumer {
    fn name(&self) -> String {
        "KafkaConsumer".into()
    }

    fn responds_to(&self, _message: &String) -> bool {
        false
    }

    async fn perform(&self, _message: &String) -> Result<Vec<String>, Error> {
        Ok(vec![])
    }

    fn schedule(&self) -> Option<String> {
        None
    }
}

async fn attach_and_handle() -> Result<(), Error> {
    let config = &config::config();
    let topic = config.topic.clone();
    let group = config.group.clone();
    let brokers = config.broker.clone();
    let consumer = create_consumer_and_subscribe(topic, group, brokers)?;

    loop {
        match consumer.recv().await {
            Ok(message) => {
                if let Ok(payload) = extract_metadata_and_deserialize(&message).await {
                    if let Ok(_) = handle(payload).await {
                        if consumer
                            .commit_message(&message, CommitMode::Async)
                            .is_err()
                        {
                            log::error!(
                                "Problems trying to commit kafka messager (it'll probrably make the consumption unstable)"
                            );
                        }
                    }
                }
            }

            Err(e) => {
                log::error!("Problems trying to receive message from Kafka: {}", e);
                break Ok(());
            }
        }
    }
}

async fn handle(payload: String) -> Result<(), Error> {
    Ok(log::info!("----> : {}", payload))
}

fn create_consumer_and_subscribe(
    topic: String,
    group: String,
    brokers: String,
) -> Result<StreamConsumer, Error> {
    let consumer: StreamConsumer = ClientConfig::new()
        .set("group.id", group)
        .set("bootstrap.servers", brokers)
        .set("auto.offset.reset", "latest")
        .set("enable.auto.commit", "false")
        .create()
        .or(helper::create_result_error(
            "Problems trying to create Kafka Consumer",
        ))?;

    consumer
        .subscribe(&[topic.as_str()])
        .or(helper::create_result_error(
            "Problems trying to subscribe Kafka Consumer",
        ))?;

    Ok(consumer)
}

async fn extract_metadata_and_deserialize(message: &BorrowedMessage<'_>) -> Result<String, Error> {
    let serialized = message.payload().ok_or(helper::create_error(
        "Problems trying to fetch Kafka Message".into(),
    ))?;

    let payload = serde_cbor::from_slice::<String>(serialized).or(helper::create_result_error(
        "Problems trying to deserialize(cbor) Kafka Message",
    ))?;

    Ok(payload)
}
