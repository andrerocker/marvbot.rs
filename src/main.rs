mod marv;

use dotenv::dotenv;
use env_logger;
use log;
use log::info;
use marv::config;
use marv::config::MarvSetup;
use marv::network;
use marv::plugins;
use prometheus_exporter::{self};
use std::io::{self, prelude::*};

fn initialize() -> MarvSetup {
    dotenv().ok();
    env_logger::init();

    prometheus_exporter::start("127.0.0.1:9184".parse().unwrap())
        .expect("Problems trying to initialize metrics");

    let setup = config::read_configuration()
        .expect("Problems trying to process Marv.toml configuration file");

    let hostname = setup.config.hostname.clone();
    log::info!("Initializing marvbot - {}", hostname);

    return setup;
}

fn main() -> io::Result<()> {
    let setup = initialize();
    let mut plugins =
        plugins::default(&setup).expect("Problems trying to initialize default plugins");

    network::stream(setup, |writer, protocol| {
        plugins::dispatch(&mut plugins, &protocol, |response: String| {
            info!("Sending response to the server: '{}'", response.trim());
            writer
                .write_all(response.as_bytes())
                .expect("Problems trying to write to the network")
        });
    });

    Ok(())
}
