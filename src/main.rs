mod marv;

use dotenv::dotenv;
use env_logger;
use log;
use marv::config;
use marv::config::MarvSetup;
use marv::network;
use marv::plugins;
use marv::plugins::Plugin;
use marv::plugins::helper;
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
    let plugins_names = helper::join(&plugins, ", ");

    Ok((setup, plugins, plugins_names))
}

fn single(setup: MarvSetup, mut plugins: Vec<Box<dyn Plugin>>) -> Result<()> {
    network::single::stream(setup, |writer, protocol| {
        plugins::dispatch(&mut plugins, &protocol, |response: String| {
            Ok(writer.write_all(response.as_bytes())?)
        });
    })
}

fn threaded(setup: MarvSetup, mut plugins: Vec<Box<dyn Plugin>>) -> Result<()> {
    network::single::stream(setup, |writer, protocol| {
        plugins::dispatch(&mut plugins, &protocol, |response: String| {
            Ok(writer.write_all(response.as_bytes())?)
        });
    })
}

fn main() -> Result<()> {
    let (setup, plugins, plugins_names) = initialize()?;

    log::info!(
        "Initializing Marvbot: {} plugins: {}",
        setup.config.hostname.clone(),
        plugins_names
    );

    single(setup, plugins)
}
