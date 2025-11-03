mod marv;

use dotenv::dotenv;
use env_logger;
use log;
use log::info;
use marv::config;
use marv::config::MarvSetup;
use marv::network;
use marv::plugins;
use marv::plugins::Plugin;
use marv::plugins::helper;
use prometheus_exporter::{self};
use std::io;
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

fn single(setup: MarvSetup, mut plugins: Vec<Box<dyn Plugin>>) -> io::Result<()> {
    network::single::stream(setup, |writer, protocol| {
        plugins::dispatch(&mut plugins, &protocol, |response: String| {
            Ok(writer.write_all(response.as_bytes())?)
        });
    })
}

fn threaded(setup: MarvSetup) -> io::Result<()> {
    network::threaded::stream(setup)
}

pub(crate) fn main() -> io::Result<()> {
    let (setup, plugins, plugins_names) = initialize()?;

    log::info!(
        "Initializing Marvbot: {} plugins: {}",
        setup.config.hostname.clone(),
        plugins_names
    );

    match setup.config.mode.as_str() {
        "thread" => threaded(setup),
        "single" => single(setup, plugins),
        _ => Ok(info!("You need to expecify a execution mode (config.mode)")),
    }
}
