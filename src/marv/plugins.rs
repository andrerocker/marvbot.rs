pub mod core;
pub mod helper;
pub mod kafka;
pub mod todo;

use super::metrics::MARV_PLUGIN_HIT_COUNTER;
use async_trait::async_trait;
use core::{channel::Channel, hello::Hello, log::Logger, login::Login, pong::Pong};
use kafka::{consumer::KafkaConsumer, producer::KafkaProducer};
use std::io::Error;
use todo::Todo;

pub type DynamicPlugin = Box<dyn Plugin>;
pub type DynamicPluginVec = Vec<DynamicPlugin>;

#[async_trait]
pub trait Plugin {
    fn name(&self) -> String;
    async fn is_enabled(&self, message: &String) -> bool;
    async fn perform(&mut self, message: &String) -> Result<Vec<String>, Error>;
}

pub fn default() -> Result<DynamicPluginVec, Error> {
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
    let detect = |name: &str| plugins.iter().find(|p| p.name() == name);

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

pub async fn dispatch<F: FnMut(String) -> Result<(), Error>>(
    plugins: &mut DynamicPluginVec,
    protocol: &String,
    mut callback: F,
) {
    for plugin in plugins.iter_mut() {
        if plugin.is_enabled(&protocol).await {
            MARV_PLUGIN_HIT_COUNTER
                .with_label_values(&[&plugin.name()])
                .inc();

            match plugin.perform(&protocol).await {
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

pub async fn dispatch_async<F: AsyncFnMut(String) -> Result<(), Error>>(
    plugins: &mut DynamicPluginVec,
    protocol: &String,
    mut callback: F,
) {
    for plugin in plugins.iter_mut() {
        if plugin.is_enabled(&protocol).await {
            MARV_PLUGIN_HIT_COUNTER
                .with_label_values(&[&plugin.name()])
                .inc();

            match plugin.perform(&protocol).await {
                Ok(response) => {
                    for result in response {
                        log::info!("Sending response to the server: '{}'", result.trim());
                        match callback(result).await {
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
