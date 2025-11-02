use crate::marv::config::MarvSetup;
use log::info;
use std::{
    io::{self, BufReader, BufWriter, prelude::*},
    net::TcpStream,
    sync::mpsc,
    thread::{self, JoinHandle},
};

pub fn stream<F: FnMut(&mut BufWriter<&TcpStream>, &String)>(
    setup: MarvSetup,
) -> io::Result<JoinHandle<io::Result<()>>> {
    let stream = TcpStream::connect(&setup.config.hostname)?;
    // let mut writer = BufWriter::new(&stream);
    let (writer, rx) = mpsc::channel::<String>();

    let thread = thread::spawn(move || {
        let mut protocol = String::new();
        let mut reader = BufReader::new(&stream);

        loop {
            if let Ok(bytes_read) = reader.read_line(&mut protocol) {
                if bytes_read == 0 {
                    info!("Connection closed");
                    break Ok(());
                }

                match writer.send(protocol.to_string()) {
                    Ok(_) => todo!(),
                    Err(error) => info!("Problems trying to send a threaded message: {}", error),
                }

                protocol.clear();
            }
        }
    });

    Ok(thread)
}
