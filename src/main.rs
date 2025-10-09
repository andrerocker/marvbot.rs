use std::io::{self, BufReader, prelude::*};
use std::net::TcpStream;

trait Plugin {
    fn check(&self, message: &String) -> bool;
    fn perform(&self, message: &String) -> Vec<String>;
}

struct Login {}
struct Pong {}

impl Plugin for Login {
    fn check(&self, message: &String) -> bool {
        return message.contains("Could not resolve your hostname");
    }

    fn perform(&self, _: &String) -> Vec<String> {
        return vec!["NICK andrerocker\r\n".to_string(), "USER andrerocker * * :Andre\r\n".to_string()];
    }
}

impl Plugin for Pong {
    fn check(&self, message: &String) -> bool {
        return message.contains("PING");
    }

    fn perform(&self, message: &String) -> Vec<String> {
        let code: String = message.split_whitespace().collect::<Vec<&str>>().last().expect("BUMM").to_string().chars().skip(1).collect();
        return vec![format!("PONG :{}\r\n", code)];
    }
}

fn process(message: String, plugins: &Vec<Box<dyn Plugin>>, mut stream: &TcpStream) -> Result<(), Box<dyn std::error::Error>> {
    let candidates = plugins.into_iter().filter(|&plugin| plugin.check(&message));
    let messages: Vec<String> = candidates.map(|plugin| plugin.perform(&message)).flatten().collect();

    for message in &messages {
        let _ = stream.write_all(message.as_bytes());
    }

    Ok(())
}

fn main() -> io::Result<()> {
    let stream = TcpStream::connect("localhost:6667")?;
    let mut reader = BufReader::new(&stream);
    let plugins: Vec<Box<dyn Plugin>> = vec![Box::new(Login{}), Box::new(Pong{})];
    
    loop {
        let mut line = String::new();
        let bytes_read = reader.read_line(&mut line)?;

        if bytes_read == 0 {
            break Ok(());
        }
        
        println!("Received: {}", line);
        let _ = process(line, &plugins, &stream);
    }
}

#[cfg(test)]
mod test;