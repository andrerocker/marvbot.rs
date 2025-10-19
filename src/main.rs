mod marv;

use env_logger;
use log;
use marv::config;
use marv::config::MarvSetup;
use marv::network;
use marv::plugins;
use prometheus_exporter::{self};
use std::io::{self, prelude::*};

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
    let mut plugins = plugins::default(&setup);

    network::stream(setup, |writer, protocol| {
        plugins::dispatch(&mut plugins, &protocol, |response: String| {
            writer.write_all(response.as_bytes()).unwrap()
        });
    });

    Ok(())
}
