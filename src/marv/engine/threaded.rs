use crate::marv::{
    config::{self},
    plugins::{self, helper},
};
use log::info;
use std::{
    io::{self, BufReader, BufWriter, prelude::*},
    net::TcpStream,
    sync::mpsc::{self, Receiver, Sender},
    thread::{self},
};

pub fn stream() -> io::Result<()> {
    let config = &config::MARV.config;
    let stream = TcpStream::connect(config.hostname.clone())?;
    let mut threads = Vec::new();

    let (network_sender, network_receiver) = mpsc::channel::<String>();
    let (plugin_sender, plugin_receiver) = mpsc::channel::<String>();

    let reader_stream = stream.try_clone()?;
    threads.push(thread::spawn(move || {
        reader_handler(reader_stream, network_sender)
    }));

    let writer_stream = stream.try_clone()?;
    threads.push(thread::spawn(move || {
        writer_handler(writer_stream, plugin_receiver)
    }));

    threads.push(thread::spawn(move || {
        plugin_handler(network_receiver, plugin_sender)
    }));

    for handle in threads.into_iter() {
        handle.join().unwrap();
    }

    Ok(())
}

fn reader_handler(stream: TcpStream, sender: Sender<String>) {
    let mut protocol = String::new();
    let mut reader = BufReader::new(stream);

    loop {
        if let Ok(bytes_read) = reader.read_line(&mut protocol) {
            if bytes_read == 0 {
                info!("Connection closed");
                break;
            }

            if let Err(error) = sender.send(protocol.to_string()) {
                info!("Problems trying to send queue a message: {}", error);
                break;
            }

            protocol.clear();
        }
    }
}

fn writer_handler(stream: TcpStream, plugin_input: Receiver<String>) {
    let mut writer = BufWriter::new(stream);

    loop {
        if let Ok(received) = plugin_input.recv() {
            if let Err(error) = writer.write(received.as_bytes()) {
                log::error!("Problems writing data to the stream: {}", error);
                break;
            } else {
                writer.flush().unwrap()
            }
        } else {
            log::error!("Problems trying to fetch plugin input");
            break;
        }
    }
}

fn plugin_handler(network_input: Receiver<String>, network_output: Sender<String>) {
    let mut plugins = plugins::default().unwrap();

    loop {
        match network_input.recv() {
            Ok(message) => {
                plugins::dispatch(
                    &mut plugins,
                    &message,
                    |response: String| match network_output.send(response) {
                        Ok(_) => Ok(()),
                        Err(error) => helper::create_result_error(
                            format!(
                                "Problems sending message to network_output_queue: {}",
                                error
                            )
                            .as_str(),
                        ),
                    },
                );
            }
            Err(error) => {
                log::error!("Problems to fetch network input: {}", error);
                break;
            }
        }
    }
}
