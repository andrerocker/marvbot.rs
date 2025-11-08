mod marv;

use env_logger;
use log;
use log::info;
use marv::config;
use marv::engine;
use marv::plugins::helper;
use prometheus_exporter;
use std::io;
use std::io::Result;

fn initialize() -> Result<()> {
    env_logger::init();
    prometheus_exporter::start("127.0.0.1:9184".parse().unwrap()).or_else(
        helper::create_closure_error("Can't initialize prometheus exporter"),
    )?;

    Ok(())
}
pub fn main() -> io::Result<()> {
    initialize()?;
    let config = &config::CONFIG.config;

    match config.mode.as_str() {
        "event" => engine::event::stream(),
        "single" => engine::single::stream(),
        "thread" => engine::threaded::stream(),
        _ => Ok(info!(
            "Set a execution mode on the configuration file [thread|event|single]"
        )),
    }
}
