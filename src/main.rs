mod marv;

use env_logger;
use marv::engine;
use marv::plugins::helper;
use prometheus_exporter;
use std::io;
use std::io::Result;

fn initialize() -> Result<()> {
    env_logger::init();
    prometheus_exporter::start("127.0.0.1:9184".parse().unwrap()).or_else(
        helper::create_closure_error("Can't initialize Prometheus Exporter"),
    )?;

    Ok(())
}
pub fn main() -> io::Result<()> {
    initialize()?;
    engine::start()
}
