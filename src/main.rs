mod marv;

use dotenv::dotenv;
use env_logger;
use log;
use log::info;
use marv::config;
use marv::engine;
use marv::plugins;
use prometheus_exporter;
use std::io;
use std::io::Result;
use std::io::prelude::*;

fn initialize() -> Result<()> {
    dotenv().ok();
    env_logger::init();

    prometheus_exporter::start("127.0.0.1:9184".parse().unwrap())
        .expect("Problems trying to initialize Metrics");

    Ok(())
}
pub fn main() -> io::Result<()> {
    initialize()?;
    let config = &config::CONFIG.config;

    match config.mode.as_str() {
        "thread" => engine::threaded::stream(),
        "event" => engine::event::stream(),
        "single" => engine::single::stream(|writer, protocol| {
            let mut plugins = plugins::default().unwrap();
            plugins::dispatch(&mut plugins, &protocol, |response: String| {
                Ok(writer.write_all(response.as_bytes())?)
            });
        }),
        _ => Ok(info!("You need to expecify a execution mode")),
    }
}
