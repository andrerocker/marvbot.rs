use crate::marv::plugins;
use marv_api::config;
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader, BufWriter},
    net::TcpSocket,
};

pub async fn initialize() {
    env_logger::init();

    prometheus_exporter::start("127.0.0.1:9184".parse().unwrap())
        .expect("Problems trying to initialize Prometheus Exporter");

    config::MARV
        .set(config::read_configuration().unwrap())
        .expect("Problems trying to read Configuration File");

    config::POOL
        .set(config::connection_pool().await)
        .expect("Problems trying to initialize Connection Pool");
}

pub async fn execute() -> anyhow::Result<()> {
    let config = config::config();

    let addr = config.hostname.clone().parse().unwrap();
    let socket = TcpSocket::new_v4()?;

    let stream = socket.connect(addr).await?;
    let (reader, writer) = stream.into_split();

    let mut reader = BufReader::new(reader);
    let mut writer = BufWriter::new(writer);
    let mut protocol = String::new();

    let mut plugins = plugins::default();

    loop {
        if let Ok(bytes_read) = reader.read_line(&mut protocol).await {
            if bytes_read == 0 {
                log::error!("Problems trying to read from the network (connection closed)");
                break;
            }

            for result in plugins::dispatch(&mut plugins, &protocol).await? {
                if let Err(error) = writer.write_all(result.as_bytes()).await {
                    log::error!("Problems trying to write to the network: {}", error);
                    break;
                };
            }

            if let Err(error) = writer.flush().await {
                log::error!("Problems trying to flush data to the network: {}", error);
                break;
            }

            protocol.clear();
        }
    }

    Ok(())
}
