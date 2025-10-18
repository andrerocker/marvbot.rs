mod marv;

use env_logger;
use log;
use marv::config;
use marv::plugins;
use prometheus_exporter::prometheus::register_counter;
use prometheus_exporter::prometheus::register_counter_vec;
use prometheus_exporter::{self};
use std::io::{self, BufReader, BufWriter, prelude::*};
use std::net::TcpStream;

fn main() -> io::Result<()> {
    env_logger::init();

    let setup = config::read_configuration().unwrap();
    let hostname = setup.config.hostname.clone();
    log::info!("Initializing marvbot - {}", hostname);

    let stream = TcpStream::connect(&hostname).unwrap();
    let mut reader = BufReader::new(&stream);
    let mut writer = BufWriter::new(&stream);

    let mut protocol = String::new();
    let mut plugins = plugins::default(&setup);

    let binding = "127.0.0.1:9184".parse().unwrap();
    prometheus_exporter::start(binding).unwrap();
    let dispatch_counter = register_counter_vec!(
        "marv_plugin_dispatch_counter",
        "Used to track how many requests was made to this call",
        &["type"]
    )
    .unwrap();

    loop {
        if let Ok(bytes_read) = reader.read_line(&mut protocol) {
            if bytes_read == 0 {
                break;
            }

            for plugin in plugins.iter_mut() {
                if plugin.is_enabled(&protocol) {
                    dispatch_counter.with_label_values(&["all"]).inc();
                    dispatch_counter.with_label_values(&[&plugin.name()]).inc();

                    for result in plugin.perform(&protocol) {
                        writer.write_all(result.as_bytes())?;
                    }
                }
            }

            writer.flush()?;
            protocol.clear();
        }
    }

    Ok(())
}
