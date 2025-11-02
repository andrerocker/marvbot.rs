use crate::marv::config::MarvSetup;
use log::info;
use std::{
    io::{BufReader, BufWriter, Error, prelude::*},
    net::TcpStream,
};

pub fn stream<F: FnMut(&mut BufWriter<&TcpStream>, &String)>(
    setup: MarvSetup,
    mut handle: F,
) -> Result<(), Error> {
    let stream = TcpStream::connect(&setup.config.hostname)
        .expect("Problems trying to connect to the server");

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
