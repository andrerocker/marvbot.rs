mod marv;

use env_logger;
use log;
use marv::config;
use marv::plugins;
use std::io::{self, BufReader, BufWriter, prelude::*};
use std::net::TcpStream;

fn main() -> io::Result<()> {
    env_logger::init();
    let marv_setup = config::read_configuration().unwrap();
    let hostname = marv_setup.config.hostname.clone();
    log::info!("Initializing marvbot - {}", hostname);

    let plugins = plugins::default(&marv_setup);
    let stream = TcpStream::connect(hostname)?;
    let mut reader = BufReader::new(&stream);
    let mut writer = BufWriter::new(&stream);
    let mut protocol = String::new();

    loop {
        if let Ok(bytes_read) = reader.read_line(&mut protocol) {
            if bytes_read == 0 {
                break Ok(());
            }

            for plugin in &plugins {
                if plugin.is_enabled(&protocol) {
                    for result in plugin.perform(&protocol) {
                        writer.write_all(result.as_bytes())?;
                    }
                }
            }

            writer.flush()?;
            protocol.clear();
        }
    }
}
