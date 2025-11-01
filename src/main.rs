mod marv;

use dotenv::dotenv;
use env_logger;
use log;
use marv::config;
use marv::config::MarvSetup;
use marv::network;
use marv::plugins;
use marv::plugins::Plugin;
use prometheus_exporter::{self};
use std::io::Result;
use std::io::prelude::*;

fn initialize() -> Result<(MarvSetup, Vec<Box<dyn Plugin>>, String)> {
    dotenv().ok();
    env_logger::init();

    prometheus_exporter::start("127.0.0.1:9184".parse().unwrap())
        .expect("Problems trying to initialize Metrics");

    let setup = config::read_configuration()
        .expect("Problems trying to process Marv.toml configuration file");

    let plugins = plugins::default(&setup)?;

    let plugins_names = plugins
        .iter()
        .map(|current| current.name())
        .collect::<Vec<String>>()
        .join(",");

    Ok((setup, plugins, plugins_names))
}

fn main() -> Result<()> {
    let (setup, mut plugins, plugins_names) = initialize()?;
    let hostname = setup.config.hostname.clone();

    log::info!(
        "Initializing Marvbot: {} plugins: {}",
        hostname,
        plugins_names
    );
    network::stream(setup, |writer, protocol| {
        plugins::dispatch(&mut plugins, &protocol, |response: String| {
            Ok(writer.write_all(response.as_bytes())?)
        });
    })
}
