use lazy_static::lazy_static;
use prometheus::{IntCounter, IntCounterVec, register_int_counter, register_int_counter_vec};

lazy_static! {
    pub static ref MARV_PLUGIN_HIT_COUNTER: IntCounterVec = register_int_counter_vec!(
        "marv_plugin_hit_counter",
        "Number of calls made to a plugin",
        &["type"]
    )
    .expect("Problems trying to initialize Plugin HIT Counter");
    pub static ref MARV_PLUGIN_KAFKA_CONSUME_COUNTER: IntCounter = register_int_counter!(
        "marv_plugin_kafka_consumer_consume_counter",
        "Track how many messages was consumed from Kafka Consumer",
    )
    .expect("Problems trying to initialize Kafka Consumer Counter");
}
