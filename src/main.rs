mod marv;

use env_logger;
use log;
use marv::config;
use marv::config::MarvSetup;
use marv::plugins;
use prometheus_exporter::prometheus::register_counter_vec;
use prometheus_exporter::{self};
use std::io::{self, BufReader, BufWriter, prelude::*};
use std::net::TcpStream;

fn initialize() -> MarvSetup {
    env_logger::init();
    prometheus_exporter::start("127.0.0.1:9184".parse().unwrap()).unwrap();

    let setup = config::read_configuration().unwrap();
    let hostname = setup.config.hostname.clone();
    log::info!("Initializing marvbot - {}", hostname);

    return setup;
}

fn main() -> io::Result<()> {
    let setup = initialize();
    let stream = TcpStream::connect(&setup.config.hostname).unwrap();
    let mut reader = BufReader::new(&stream);
    let mut writer = BufWriter::new(&stream);

    let mut protocol = String::new();
    let mut plugins = plugins::default(&setup);

    loop {
        if let Ok(bytes_read) = reader.read_line(&mut protocol) {
            if bytes_read == 0 {
                break;
            }

            plugins::dispatch(&mut plugins, &protocol, |response: String| {
                writer.write_all(response.as_bytes()).unwrap()
            });

            writer.flush()?;
            protocol.clear();
        }
    }

    Ok(())
}
