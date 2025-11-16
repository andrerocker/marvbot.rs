pub mod core;

use marv_api::plugins::DynamicPluginVec;
use std::io;

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
    // let mut results = Vec::new();
    // let mut plugins = PLUGINS.lock().unwrap();

    // let (output, handles) = unsafe {
    //     TokioScope::scope_and_collect(|scope| {
    //         for plugin in plugins.iter_mut() {
    //             if plugin.is_enabled(&protocol) {
    //                 scope.spawn(async move { plugin.perform(&protocol).await.unwrap() });
    //             }
    //         }
    //     })
    // }
    // .await;

    // for current in &handles.iter_mut() {}

    // let mut results = Vec::new();
    // let mut handles = Vec::new();
    // let mut plugins = Arc::new(Mutex::new(vec![
    //     Logger::new(),
    //     Login::new(),
    //     Pong::new(),
    //     Channel::new(),
    //     Hello::new(),
    //     KafkaProducer::new(),
    //     KafkaConsumer::new(),
    //     Todo::new(),
    // ]));
    // let acme = plugins.lock().unwrap();

    // for plugin in acme.iter_mut() {
    //     if plugin.is_enabled(&protocol) {
    //         // let plugin = Arc::new(Mutex::new(plugin));
    //         let protocol = protocol.clone();

    //         let handle = tokio::spawn(async move {
    //             // let plugin = plugin.lock().unwrap();
    //             plugin.perform(&protocol).await.unwrap()
    //         });

    //         handles.push(handle);
    //     }
    // }

    // for handle in handles.iter_mut() {
    //     results.append(&mut handle.await.unwrap());
    // }

    let mut results = Vec::new();

    for plugin in plugins.iter_mut() {
        if plugin.is_enabled(&protocol) {
            results.append(&mut plugin.perform(&protocol).await.unwrap());
        }
    }

    Ok(results)
}
