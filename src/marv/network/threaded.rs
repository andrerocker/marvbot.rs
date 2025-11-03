use crate::marv::config::MarvSetup;
use log::info;
use std::{
    io::{self, BufReader, BufWriter, prelude::*},
    net::TcpStream,
    sync::mpsc::{self, Receiver, Sender},
    thread::{self, JoinHandle},
};

pub fn stream(setup: MarvSetup) -> io::Result<()> {
    let stream = TcpStream::connect(&setup.config.hostname)?;
    let mut reader = BufReader::new(&stream);
    let mut writer = BufWriter::new(&stream);

    let (sender, receiver) = mpsc::channel::<String>();
    let reader_thread = reader_thread(&reader, sender);
    let writer_thread = writer_thread(&writer, receiver);

    reader_thread.join();
    writer_thread.join();

    Ok(())
}

fn reader_thread(
    reader: &BufReader<&TcpStream>,
    sender: Sender<String>,
) -> JoinHandle<io::Result<()>> {
    thread::spawn(|| {
        let mut protocol = String::new();

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
    })
}

fn writer_thread(
    writer: &BufWriter<&TcpStream>,
    receiver: Receiver<String>,
) -> JoinHandle<io::Result<()>> {
    thread::spawn(|| match receiver.recv() {
        Ok(received) => writer.write(received.as_bytes()),
        Err(error) => log::error!("Problems to pick messagem: {}", error),
    })
}
