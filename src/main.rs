mod marv;

use env_logger;
use log;
use marv::plugins;
use std::io::{self, BufReader, BufWriter, prelude::*};
use std::net::TcpStream;

fn plugins_enabled() -> Vec<Box<dyn plugins::Plugin>> {
    return vec![
        Box::new(plugins::Logger {}),
        Box::new(plugins::Login {}),
        Box::new(plugins::Pong {}),
        Box::new(plugins::Channel {}),
        Box::new(plugins::Hello {}),
    ];
}

fn main() -> io::Result<()> {
    env_logger::init();

    let server = "localhost:6667";
    log::info!("Initializing marvbot - {}", server);

    let plugins = plugins_enabled();
    let stream = TcpStream::connect(server)?;
    let mut reader = BufReader::new(&stream);
    let mut writer = BufWriter::new(&stream);
    let mut protocol = String::new();

    loop {
        if let Ok(bytes_read) = reader.read_line(&mut protocol) {
            if bytes_read != 0 {
                for plugin in &plugins {
                    if plugin.is_enabled(&protocol) {
                        for result in plugin.perform(&protocol) {
                            writer.write_all(result.as_bytes())?;
                        }
                    }
                }

                let _ = writer.flush();
                protocol.clear();
            } else {
                break Ok(());
            }
        }
    }
}
