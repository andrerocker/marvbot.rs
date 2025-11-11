use std::io::Result;

use crate::marv::config;
use crate::marv::plugins;
use crate::marv::plugins::kafka::consumer_v2::KafkaV2Consumer;
use log::info;
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader, BufWriter},
    net::TcpSocket,
};

#[tokio::main]
pub async fn stream() -> Result<()> {
    let config = &config::MARV.config;

    let addr = config.hostname.clone().parse().unwrap();
    let socket = TcpSocket::new_v4()?;

    let stream = socket.connect(addr).await?;
    let (reader, writer) = stream.into_split();

    let mut reader = BufReader::new(reader);
    let mut writer = BufWriter::new(writer);
    let mut protocol = String::new();

    let mut plugins = plugins::default().unwrap();
    plugins.push(KafkaV2Consumer::new());

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
