mod marv;

use env_logger;
use log;
use marv::config;
use marv::plugins;
use std::io::{self, BufReader, BufWriter, prelude::*};
use std::net::TcpStream;

fn main() -> io::Result<()> {
    env_logger::init();

    match config::read_configuration() {
        Ok(setup) => {
            let hostname = setup.config.hostname.clone();
            log::info!("Initializing marvbot - {}", hostname);

            match TcpStream::connect(&hostname) {
                Ok(stream) => {
                    let mut reader = BufReader::new(&stream);
                    let mut writer = BufWriter::new(&stream);

                    let mut protocol = String::new();
                    let mut plugins = plugins::default(&setup);

                    loop {
                        if let Ok(bytes_read) = reader.read_line(&mut protocol) {
                            if bytes_read == 0 {
                                break;
                            }

                            for plugin in plugins.iter_mut() {
                                if plugin.is_enabled(&protocol) {
                                    for result in plugin.perform(&protocol) {
                                        writer.write_all(result.as_bytes())?;
                                    }
                                }
                            }

                            writer.flush()?;
                            protocol.clear();
                        }
                    }
                }
                Err(e) => {
                    log::error!(
                        "IRC Server: Fail trying to connect to: {}, {}",
                        &hostname,
                        e
                    )
                }
            }
        }
        Err(e) => {
            log::error!(
                "Config: Fail trying to load Marv.toml configuration file: {}",
                e
            )
        }
    }

    Ok(())
}
