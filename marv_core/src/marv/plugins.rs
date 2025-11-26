pub mod core;
pub mod dispatch;
pub mod scheduled;

use core::{
    channel::Channel, hello::Hello, hello_fast::HelloFast, hello_slow::HelloSlow, log::Logger,
    login::Login, pong::Pong, spam::Spam,
};
use marv_api::plugins::{DynamicPlugin, DynamicPluginVec};
use marv_plugins::{
    ask_chatgpt::AskChatGPT,
    kafka::{consumer::KafkaConsumer, producer::KafkaProducer},
    todo::Todo,
};
use once_cell::sync::OnceCell;

static PLUGINS: OnceCell<DynamicPluginVec> = OnceCell::new();

fn default_plugins() -> &'static DynamicPluginVec {
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
            HelloFast::new(),
            HelloSlow::new(),
            AskChatGPT::new(),
            Spam::new(),
        ]
    })
}

fn default_schedulables() -> Vec<(String, &'static DynamicPlugin)> {
    let mut candidates = Vec::new();

    for plugin in crate::marv::plugins::default_plugins() {
        if let Some(schedulable) = plugin.schedule() {
            candidates.push((schedulable, plugin));
        }
    }

    candidates
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
