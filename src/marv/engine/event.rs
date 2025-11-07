use std::io::Result;

use crate::marv::config;
use log::info;
use tokio::io::AsyncBufReadExt;
use tokio::io::AsyncWriteExt;

use tokio::{
    io::{BufReader, BufWriter},
    net::TcpSocket,
};

#[tokio::main]
pub async fn stream() -> Result<()> {
    let config = &config::CONFIG.config;

    let addr = config.hostname.clone().parse().unwrap();
    let socket = TcpSocket::new_v4()?;

    let stream = socket.connect(addr).await?;
    let (reader, writer) = stream.into_split();

    tokio::spawn(async move {
        let mut reader = BufReader::new(reader);
        let mut writer = BufWriter::new(writer);
        let mut protocol = String::new();

        loop {
            if let Ok(bytes_read) = reader.read_line(&mut protocol).await {
                if bytes_read == 0 {
                    info!("Connection closed");
                    break;
                }

                // handle(&mut writer, &protocol);
                log::info!("-->> {}", protocol);

                if let Err(error) = writer.flush().await {
                    log::error!("Problems trying to flush data to the network: {}", error);
                    break;
                }
                protocol.clear();
            }
        }
    })
    .await?;

    Ok(())
}
