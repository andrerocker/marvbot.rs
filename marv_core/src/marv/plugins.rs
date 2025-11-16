pub mod core;

use marv_api::plugins::DynamicPluginVec;
use std::{
    io::{self, Error},
    pin::Pin,
};

// static PLUGINS: OnceCell<DynamicPluginVec> = OnceCell::new();

// fn plugins() -> &'static DynamicPluginVec {
//     PLUGINS.get_or_init(|| {
//         vec![
//             Logger::new(),
//             Login::new(),
//             Pong::new(),
//             Channel::new(),
//             Hello::new(),
//             KafkaProducer::new(),
//             KafkaConsumer::new(),
//             Todo::new(),
//         ]
//     })
// }

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

pub async fn dispatch(
    plugins: &mut DynamicPluginVec,
    protocol: &String,
) -> io::Result<Vec<String>> {
    let mut results = Vec::new();
    let mut handles: Vec<Pin<Box<dyn Future<Output = Result<Vec<String>, Error>> + Send>>> =
        Vec::new();

    for plugin in plugins.iter_mut() {
        if plugin.is_enabled(&protocol) {
            handles.push(plugin.perform(&protocol));
        }
    }

    for handle in handles {
        results.append(&mut handle.await.unwrap());
    }

    Ok(results)
}
