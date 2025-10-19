pub mod core;
pub mod kafka;

use kafka::{consumer::KafkaConsumer, producer::KafkaProducer};
use lazy_static::lazy_static;
use prometheus::{IntCounterVec, register_int_counter_vec};

use super::config;
use core::{channel::Channel, hello::Hello, log::Logger, login::Login, pong::Pong};

lazy_static! {
    static ref MARV_PLUGIN_HIT_COUNTER: IntCounterVec = register_int_counter_vec!(
        "marv_plugin_hit_counter",
        "Number of calls made to a plugin",
        &["type"]
    )
    .unwrap();
}

pub trait Plugin {
    fn name(&self) -> String;
    fn is_enabled(&self, message: &String) -> bool;
    fn perform(&mut self, message: &String) -> Vec<String>;
}

pub fn default(setup: &config::MarvSetup) -> Vec<Box<dyn Plugin>> {
    return vec![
        Logger::new(setup),
        Login::new(setup),
        Pong::new(setup),
        Channel::new(setup),
        Hello::new(setup),
        KafkaProducer::new(setup),
        KafkaConsumer::new(setup),
    ];
}

pub fn dispatch<F: FnMut(String)>(
    plugins: &mut Vec<Box<dyn Plugin>>,
    protocol: &String,
    mut callback: F,
) {
    for plugin in plugins.iter_mut() {
        if plugin.is_enabled(&protocol) {
            MARV_PLUGIN_HIT_COUNTER.with_label_values(&["all"]).inc();
            MARV_PLUGIN_HIT_COUNTER
                .with_label_values(&[&plugin.name()])
                .inc();

            for result in plugin.perform(&protocol) {
                callback(result);
            }
        }
    }
}
