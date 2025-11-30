use std::env;

use super::plugins::dispatch;
use super::plugins::scheduled;
use anyhow::Context;
use marv_api::config;
use tokio::{
    io::{AsyncWriteExt, BufReader, BufWriter},
    net::{TcpSocket, tcp::OwnedWriteHalf},
    sync::mpsc::{Receiver, channel},
};

pub async fn initialize() {
    let environment = env::var("RUST_ENV").unwrap_or_else(|_| "development".into());
    dotenvy::from_filename(format!(".env.{}", environment)).unwrap();
    env_logger::init();

    prometheus_exporter::start("127.0.0.1:9184".parse().unwrap())
        .expect("Problems trying to initialize Prometheus Exporter");

    config::initialize_config().await;
    config::initialize_pool().await;
}

async fn wait_and_write(
    mut writer: BufWriter<OwnedWriteHalf>,
    mut receiver: Receiver<Vec<String>>,
) -> anyhow::Result<()> {
    while let Some(responses) = receiver.recv().await {
        for response in responses {
            writer
                .write_all(response.as_bytes())
                .await
                .context("Writing date to the Network")?;

            writer
                .flush()
                .await
                .context("Flushing data to the Network")?;
        }
    }

    Ok(())
}

pub async fn execute() -> anyhow::Result<()> {
    let config = config::config();
    let socket = TcpSocket::new_v4()?;

    let addr = config
        .hostname
        .clone()
        .parse()
        .context("Creating socket addr from hostname")?;

    let stream = socket
        .connect(addr)
        .await
        .context("Creating socket to the IRC Server")?;

    let (reader, writer) = stream.into_split();
    let network_reader = BufReader::new(reader);
    let network_writer = BufWriter::new(writer);
    let (sender, receiver) = channel(10);

    let scheduler_sender = sender.clone();
    let dispatcher_sender = sender.clone();

    scheduled::spawn(scheduler_sender);
    dispatch::spawn(network_reader, dispatcher_sender);
    wait_and_write(network_writer, receiver).await?;

    Ok(())
}
