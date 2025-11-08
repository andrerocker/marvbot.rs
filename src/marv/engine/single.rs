use crate::marv::{config, plugins};
use log::info;
use std::{
    io::{BufReader, BufWriter, Error, prelude::*},
    net::TcpStream,
};

fn internal<F: FnMut(&mut BufWriter<&TcpStream>, &String)>(mut handle: F) -> Result<(), Error> {
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

            handle(&mut writer, &protocol);

            writer
                .flush()
                .expect("Problems trying to flush network data");
            protocol.clear();
        }
    }
}

pub fn stream() -> Result<(), Error> {
    internal(|writer, protocol| {
        let mut plugins = plugins::default().unwrap();
        plugins::dispatch(&mut plugins, &protocol, |response: String| {
            Ok(writer.write_all(response.as_bytes())?)
        });
    })
}
