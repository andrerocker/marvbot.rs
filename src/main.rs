mod marv;

use marv::plugins;
use std::io::{self, BufReader, BufWriter, prelude::*};
use std::net::TcpStream;

fn plugins_enabled() -> Vec<Box<dyn plugins::Plugin>> {
    return vec![
        Box::new(plugins::Login{}), 
        Box::new(plugins::Pong{})
    ];
}

fn main() -> io::Result<()> {
    let plugins = plugins_enabled();
    let stream = TcpStream::connect("localhost:6667")?;
    let mut reader = BufReader::new(&stream);
    let mut writer = BufWriter::new(&stream);

    loop {
        let mut protocol = String::new();
        let bytes_read = reader.read_line(&mut protocol)?;

        if bytes_read == 0 {
            break Ok(());
        } else {
            print!("<-- {}", protocol);

            for plugin in &plugins {
                if plugin.check(&protocol) {
                    for result in plugin.perform(&protocol) {
                        writer.write_all(result.as_bytes())?;
                    }
                }
            }

            writer.flush();
        }
    }
}