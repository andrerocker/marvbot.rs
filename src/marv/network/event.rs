use crate::marv::config::{self};
use log::info;
use std::{
    io::{BufReader, BufWriter, Error, prelude::*},
    net::TcpStream,
};

#[tokio::main]
pub async fn stream() -> Result<(), Error> {
    let config = &config::CONFIG.config;
    let stream = TcpStream::connect(config.hostname.clone())?;

    let mut reader = BufReader::new(&stream);
    let mut writer = BufWriter::new(&stream);
    let mut protocol = String::new();

    loop {
        if let Ok(bytes_read) = reader.read_line(&mut protocol) {
            if bytes_read == 0 {
                info!("Connection closed");
                break Ok(());
            }

            // handle(&mut writer, &protocol);

            writer
                .flush()
                .expect("Problems trying to flush network data");
            protocol.clear();
        }
    }
}
