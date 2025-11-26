use std::io;
use tokio::task::JoinSet;

pub async fn execute<F: AsyncFnMut(Vec<String>)>(
    protocol: &String,
    mut callback: F,
) -> io::Result<bool> {
    let mut handles = JoinSet::new();
    let plugins = crate::marv::plugins::default_plugins();

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
