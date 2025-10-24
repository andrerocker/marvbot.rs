pub mod core;
pub mod etc;

use etc::{consumer::KafkaConsumer, database::Database, producer::KafkaProducer};

use super::{config, metrics::MARV_PLUGIN_HIT_COUNTER};
use core::{channel::Channel, hello::Hello, log::Logger, login::Login, pong::Pong};

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
        Database::new(setup),
    ];
}

pub fn dispatch<F: FnMut(String)>(
    plugins: &mut Vec<Box<dyn Plugin>>,
    protocol: &String,
    mut callback: F,
) {
    for plugin in plugins.iter_mut() {
        if plugin.is_enabled(&protocol) {
            MARV_PLUGIN_HIT_COUNTER
                .with_label_values(&[&plugin.name()])
                .inc();

            for result in plugin.perform(&protocol) {
                callback(result);
            }
        }
    }
}
