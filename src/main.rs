mod marv;

use dotenv::dotenv;
use env_logger;
use log;
use log::info;
use marv::config;
use marv::network;
use marv::plugins;
use marv::plugins::Plugin;
use marv::plugins::helper;
use prometheus_exporter::{self};
use std::io;
use std::io::Result;
use std::io::prelude::*;

fn initialize() -> Result<(plugins::DynamicPluginVec, String)> {
    dotenv().ok();
    env_logger::init();

    prometheus_exporter::start("127.0.0.1:9184".parse().unwrap())
        .expect("Problems trying to initialize Metrics");

    let plugins = plugins::default()?;
    let plugins_names = helper::join(&plugins, ", ");

    Ok((plugins, plugins_names))
}

fn single(mut plugins: Vec<Box<dyn Plugin>>) -> io::Result<()> {
    network::single::stream(|writer, protocol| {
        plugins::dispatch(&mut plugins, &protocol, |response: String| {
            Ok(writer.write_all(response.as_bytes())?)
        });
    })
}

fn threaded() -> io::Result<()> {
    network::threaded::stream()
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let config = &config::CONFIG.config;
    let (plugins, plugins_names) = initialize()?;

    log::info!(
        "Initializing Marvbot: {} plugins: {}",
        config.hostname.clone(),
        plugins_names
    );

    match config.mode.as_str() {
        "thread" => threaded(),
        "single" => single(plugins),
        _ => Ok(info!("You need to expecify a execution mode")),
    }
}
