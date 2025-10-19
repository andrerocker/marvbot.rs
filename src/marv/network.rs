use super::config::MarvSetup;
use std::{
    io::{BufReader, BufWriter, prelude::*},
    net::TcpStream,
};

pub fn stream<F: FnMut(&mut BufWriter<&TcpStream>, &String)>(setup: MarvSetup, mut handle: F) {
    let stream = TcpStream::connect(&setup.config.hostname).unwrap();
    let mut reader = BufReader::new(&stream);
    let mut writer = BufWriter::new(&stream);

    let mut protocol = String::new();

    loop {
        if let Ok(bytes_read) = reader.read_line(&mut protocol) {
            if bytes_read == 0 {
                break;
            }

            handle(&mut writer, &protocol);

            writer.flush().unwrap();
            protocol.clear();
        }
    }
}
