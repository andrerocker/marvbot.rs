pub mod core;
pub mod helper;
pub mod kafka;
pub mod todo;

use super::metrics::MARV_PLUGIN_HIT_COUNTER;
use core::{channel::Channel, hello::Hello, log::Logger, login::Login, pong::Pong};
use kafka::{consumer::KafkaConsumer, producer::KafkaProducer};
use std::{
    fmt::{self, Display},
    io::Error,
};
use todo::Todo;

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

pub fn default() -> Result<Vec<Box<dyn Plugin>>, Error> {
    Ok(vec![
        Logger::new(),
        Login::new(),
        Pong::new(),
        Channel::new(),
        Hello::new(),
        KafkaProducer::new(),
        KafkaConsumer::new(),
        Todo::new(),
    ])
}

#[test]
fn test_default_plugins() -> Result<(), Box<dyn std::error::Error>> {
    let plugins = default()?;
    let detect = |name: &str| plugins.iter().find(|p| p.name().eq(&name));

    assert!(detect("Logger").is_some());
    assert!(detect("Login").is_some());
    assert!(detect("Pong").is_some());
    assert!(detect("Channel").is_some());
    assert!(detect("Hello").is_some());
    assert!(detect("KafkaProducer").is_some());
    assert!(detect("KafkaConsumer").is_some());
    assert!(detect("Todo").is_some());

    Ok(())
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
