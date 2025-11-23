pub mod core;

use core::{
    channel::Channel, hello::Hello, hello_fast::HelloFast, hello_slow::HelloSlow, log::Logger,
    login::Login, pong::Pong,
};
use marv_api::plugins::DynamicPluginVec;
use marv_plugins::{
    kafka::{consumer::KafkaConsumer, producer::KafkaProducer},
    todo::Todo,
};
use once_cell::sync::OnceCell;
use std::{
    io::{self},
    sync::mpsc::Sender,
};
use tokio::task::JoinSet;

static PLUGINS: OnceCell<DynamicPluginVec> = OnceCell::new();

fn default_plugins() -> &'static DynamicPluginVec {
    PLUGINS.get_or_init(|| {
        vec![
            Logger::new(),
            Login::new(),
            Pong::new(),
            Channel::new(),
            Hello::new(),
            // KafkaProducer::new(),
            // KafkaConsumer::new(),
            // Todo::new(),
            HelloFast::new(),
            HelloSlow::new(),
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

pub async fn dispatch<F: AsyncFnMut(Vec<String>)>(
    protocol: &String,
    mut callback: F,
) -> io::Result<bool> {
    let plugins = default_plugins();
    let mut sets = JoinSet::new();

    for plugin in plugins {
        if plugin.is_enabled(&protocol) {
            let protocol = protocol.clone();
            sets.spawn(async move { plugin.perform(&protocol).await });
        }
    }

    while let Some(join_response) = sets.join_next().await {
        match join_response {
            Ok(response) => callback(response.unwrap()).await,
            Err(error) => log::error!("Problems trying to join task and add to results: {error}"),
        }
    }

    Ok(true)
}
