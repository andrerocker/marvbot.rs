use std::io;
use tokio::{
    io::{AsyncBufReadExt, BufReader},
    net::tcp::OwnedReadHalf,
    sync::mpsc::Sender,
    task::JoinSet,
};

pub fn spawn(
    mut reader: BufReader<OwnedReadHalf>,
    sender: Sender<Vec<String>>,
) -> tokio::task::JoinHandle<()> {
    tokio::task::spawn(async move {
        let mut protocol = String::new();
        loop {
            if let Ok(bytes_read) = reader.read_line(&mut protocol).await {
                if bytes_read == 0 {
                    log::error!("Problems trying to read from the network (connection closed)");
                    break;
                }

                let sender = sender.clone();
                let dispatched = execute(&protocol, async move |responses: Vec<String>| {
                    sender.send(responses).await.unwrap();
                })
                .await;

                if let Err(error) = dispatched {
                    log::error!("Problems trying to dispatch plugins: {}", error);
                }

                protocol.clear();
            }
        }
    })
}

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
