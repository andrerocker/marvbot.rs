use std::env;

use crate::marv::plugins;
use anyhow::Context;
use marv_api::config;
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader, BufWriter},
    net::{
        TcpSocket,
        tcp::{OwnedReadHalf, OwnedWriteHalf},
    },
    sync::mpsc::{Receiver, Sender, channel},
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

fn spawn_scheduled_plugins(sender: Sender<Vec<String>>) -> tokio::task::JoinHandle<()> {
    // TODO: Improve error handling here!
    tokio::task::spawn(async move {
        plugins::scheduled::execute(async move |responses: Vec<String>| {
            sender.send(responses).await.unwrap();
        })
        .await
        .unwrap()
    })
}

fn spawn_dispatcher_plugins(
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
                let dispatched =
                    plugins::dispatch::execute(&protocol, async move |responses: Vec<String>| {
                        sender.send(responses).await.unwrap();
                    })
                    .await;

                if let Err(error) = dispatched {
                    log::error!(
                        "Problems trying to dispatch a call to the plugins: {}",
                        error
                    );
                }

                protocol.clear();
            }
        }
    })
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

    spawn_scheduled_plugins(scheduler_sender);
    spawn_dispatcher_plugins(network_reader, dispatcher_sender);
    wait_and_write(network_writer, receiver).await?;

    Ok(())
}
