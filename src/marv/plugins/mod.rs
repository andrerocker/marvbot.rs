pub mod core;
pub mod etc;
pub mod helper;

use super::{config, metrics::MARV_PLUGIN_HIT_COUNTER};
use core::{channel::Channel, hello::Hello, log::Logger, login::Login, pong::Pong};
use etc::{consumer::KafkaConsumer, producer::KafkaProducer, todo::Todo};
use std::{
    fmt::{self, Display},
    io::Error,
};

pub trait Plugin {
    fn name(&self) -> String;
    fn is_enabled(&self, message: &String) -> bool;
    fn perform(&mut self, message: &String) -> Result<Vec<String>, Error>;
}

impl Display for dyn Plugin {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

pub fn default(setup: &config::MarvSetup) -> Result<Vec<Box<dyn Plugin>>, Error> {
    Ok(vec![
        Logger::new(setup),
        Login::new(setup),
        Pong::new(setup),
        Channel::new(setup),
        Hello::new(setup),
        KafkaProducer::new(setup),
        KafkaConsumer::new(setup),
        Todo::new(setup),
    ])
}

pub fn dispatch<F: FnMut(String) -> Result<(), Error>>(
    plugins: &mut Vec<Box<dyn Plugin>>,
    protocol: &String,
    mut callback: F,
) {
    for plugin in plugins.iter_mut() {
        if plugin.is_enabled(&protocol) {
            MARV_PLUGIN_HIT_COUNTER
                .with_label_values(&[&plugin.name()])
                .inc();

            match plugin.perform(&protocol) {
                Ok(response) => {
                    for result in response {
                        log::info!("Sending response to the server: '{}'", result.trim());
                        match callback(result) {
                            Ok(_) => continue,
                            Err(error) => {
                                log::error!("Problems trying to call callback: {}", error)
                            }
                        }
                    }
                }
                Err(error) => {
                    log::error!("Problems processing plugin: {}", error)
                }
            }
        }
    }
}
