use crate::marv::plugins::{self};
use marv_api::config;
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader, BufWriter},
    net::TcpSocket,
};

pub async fn initialize() {
    env_logger::init();

    prometheus_exporter::start("127.0.0.1:9184".parse().unwrap())
        .expect("Problems trying to initialize Prometheus Exporter");

    config::set_config(
        config::read_configuration().expect("Problems trying to read Configuration File"),
    );

    config::set_pool(config::connection_pool().await);
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

    loop {
        if let Ok(bytes_read) = reader.read_line(&mut protocol).await {
            if bytes_read == 0 {
                log::error!("Problems trying to read from the network (connection closed)");
                break;
            }

            let dispached = plugins::dispatch(&protocol, async |items: Vec<String>| {
                for current in items {
                    if let Err(error) = writer.write_all(current.as_bytes()).await {
                        log::error!("Problems trying to write data to the network: {}", error);
                    }

                    if let Err(error) = writer.flush().await {
                        log::error!("Problems trying to flush data to the network: {}", error);
                    }
                }
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

    Ok(())
}
