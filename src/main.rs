mod marv;

use marv::plugins;
use std::io::{self, BufReader, prelude::*};
use std::net::TcpStream;

fn process(protocol: String, plugins: &Vec<Box<dyn plugins::Plugin>>, mut stream: &TcpStream) -> Result<(), Box<dyn std::error::Error>> {
    let candidates = plugins.into_iter().filter(|&plugin| plugin.check(&protocol));

    for plugin in candidates {
        for result in plugin.perform(&protocol) {
            stream.write_all(result.as_bytes())?;
        }
    }

    Ok(())
}

fn main() -> io::Result<()> {
    let stream = TcpStream::connect("localhost:6667")?;
    let mut reader = BufReader::new(&stream);
    let plugins: Vec<Box<dyn plugins::Plugin>> = vec![
        Box::new(plugins::Login{}), 
        Box::new(plugins::Pong{})
    ];
    
    loop {
        let mut line = String::new();
        let bytes_read = reader.read_line(&mut line)?;

        if bytes_read == 0 {
            break Ok(());
        }
        
        print!("<-- {}", line);
        let _ = process(line, &plugins, &stream);
    }
}

// #[cfg(test)]
// mod test;