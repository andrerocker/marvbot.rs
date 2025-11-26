pub mod core;

use cargo::util::context::StringList;
use core::{
    channel::Channel, hello::Hello, hello_fast::HelloFast, hello_slow::HelloSlow, log::Logger,
    login::Login, pong::Pong,
};
use marv_api::plugins::DynamicPluginVec;
use marv_plugins::{
    ask_chatgpt::AskChatGPT,
    kafka::{consumer::KafkaConsumer, producer::KafkaProducer},
    todo::Todo,
};
use once_cell::sync::OnceCell;
use std::io::{self};
use tokio::{sync::mpsc, task::JoinSet};
use tokio_cron_scheduler::{Job, JobScheduler};

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
    let mut handles = JoinSet::new();

    for plugin in plugins {
        if plugin.responds_to(&protocol) {
            let protocol = protocol.clone();
            handles.spawn(async move { plugin.perform(&protocol).await });
        }
    }

    while let Some(response) = handles.join_next().await {
        match response {
            Ok(response) => match response {
                Ok(response) => callback(response).await,
                Err(error) => {
                    log::error!("Problems trying calling plugin: {error}")
                }
            },
            Err(error) => log::error!("Problems trying to join next task: {error}"),
        }
    }

    Ok(true)
}

pub async fn schedule<F: AsyncFnMut(Vec<String>)>(mut callback: F) -> io::Result<()> {
    let mut candidates = Vec::new();
    let scheduler = JobScheduler::new().await.unwrap();
    let (writer, mut receiver) = mpsc::channel::<Vec<String>>(10);

    for plugin in default_plugins() {
        if let Some(schedulable) = plugin.schedule() {
            candidates.push((schedulable, plugin));
        }
    }

    for (appointment, plugin) in candidates {
        let writer = writer.clone();
        let job = Job::new_async(appointment, move |_uuid, _l| {
            let writer = writer.clone();
            Box::pin(async move {
                let response = plugin.perform(&"schedule".to_string()).await.unwrap();
                writer.send(response).await.unwrap();
                ()
            })
        })
        .unwrap();

        scheduler.add(job).await.unwrap();
    }

    scheduler.start().await.unwrap();

    while let Some(msg) = receiver.recv().await {
        log::info!("----------------------------> sched: response: {:?}", msg);
        callback(msg).await;
    }

    Ok(())
}
