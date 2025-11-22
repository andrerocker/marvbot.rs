pub mod core;

use core::{channel::Channel, hello::Hello, log::Logger, login::Login, pong::Pong};
use marv_api::plugins::DynamicPluginVec;
use marv_plugins::{
    kafka::{consumer::KafkaConsumer, producer::KafkaProducer},
    todo::Todo,
};
use once_cell::sync::OnceCell;
use std::{
    io::{self, Error},
    pin::Pin,
};

static PLUGINS: OnceCell<DynamicPluginVec> = OnceCell::new();

fn plugins() -> &'static DynamicPluginVec {
    PLUGINS.get_or_init(|| {
        vec![
            Logger::new(),
            Login::new(),
            Pong::new(),
            Channel::new(),
            Hello::new(),
            KafkaProducer::new(),
            KafkaConsumer::new(),
            Todo::new(),
        ]
    })
}

// #[test]
// fn test_default_plugins() -> Result<(), Box<dyn std::error::Error>> {
//     let plugins = &PLUGINS;
//     let detect = |name: &str| plugins.iter().find(|p| p.name() == name);

//     assert!(detect("Logger").is_some());
//     assert!(detect("Login").is_some());
//     assert!(detect("Pong").is_some());
//     assert!(detect("Channel").is_some());
//     assert!(detect("Hello").is_some());
//     assert!(detect("KafkaProducer").is_some());
//     assert!(detect("KafkaConsumer").is_some());
//     assert!(detect("Todo").is_some());

//     Ok(())
// }

pub async fn dispatch(protocol: &String) -> io::Result<Vec<String>> {
    let plugins = plugins();
    let mut results = Vec::new();

    for plugin in plugins {
        if plugin.is_enabled(&protocol) {
            let mut result = plugin.perform(protocol).await.unwrap();
            results.append(&mut result);
        }
    }

    Ok(results)
}
