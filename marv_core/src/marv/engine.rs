use std::env;

use crate::marv::plugins;
use marv_api::config;
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader, BufWriter},
    net::TcpSocket,
    sync::mpsc::channel,
};

pub async fn initialize() {
    let environment = env::var("RUST_ENV").unwrap_or_else(|_| "development".into());
    dotenvy::from_filename(format!(".env.{}", environment)).unwrap();
    env_logger::init();

    prometheus_exporter::start("127.0.0.1:9184".parse().unwrap())
        .expect("Problems trying to initialize Prometheus Exporter");

    config::initialize_config();
    config::initialize_pool().await;
}

pub async fn execute() -> anyhow::Result<()> {
    let config = config::config();
    let addr = config.hostname.clone().parse().unwrap();
    let socket = TcpSocket::new_v4()?;

    let stream = socket.connect(addr).await?;
    let (reader, writer) = stream.into_split();

    let mut protocol = String::new();
    let mut reader = BufReader::new(reader);
    let mut writer = BufWriter::new(writer);

    let (sender, mut receiver) = channel(10);

    let sched_sender = sender.clone();
    tokio::task::spawn(async move {
        plugins::scheduled::execute(async move |responses: Vec<String>| {
            sched_sender.send(responses).await.unwrap();
        })
        .await
        .unwrap();
    });

    let dispatch_sender = sender.clone();
    tokio::task::spawn(async move {
        loop {
            if let Ok(bytes_read) = reader.read_line(&mut protocol).await {
                if bytes_read == 0 {
                    log::error!("Problems trying to read from the network (connection closed)");
                    break;
                }

                let dispatch_sender = dispatch_sender.clone();
                let dispached =
                    plugins::dispatch::execute(&protocol, async move |responses: Vec<String>| {
                        dispatch_sender.send(responses).await.unwrap();
                    })
                    .await;

                if let Err(error) = dispached {
                    log::error!(
                        "Problems trying to dispatch a call to the plugins: {}",
                        error
                    );
                }

                protocol.clear();
            }
        }
    });

    while let Some(responses) = receiver.recv().await {
        for response in responses {
            if let Err(error) = writer.write_all(response.as_bytes()).await {
                log::error!("Problems trying to write data to the network: {}", error);
            }

            if let Err(error) = writer.flush().await {
                log::error!("Problems trying to flush data to the network: {}", error);
            }
        }
    }

    Ok(())
}
