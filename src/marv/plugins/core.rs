use regex::{self, Regex};

use super::Plugin;

pub struct Logger {}
pub struct Login {
    pub nickname: String,
}
pub struct Pong {}
pub struct Channel {
    pub channel: String,
}
pub struct Hello {}

impl Plugin for Login {
    fn is_enabled(&self, message: &String) -> bool {
        return message.contains("Could not resolve your hostname");
    }

    fn perform(&self, _: &String) -> Vec<String> {
        log::info!("--> Executando Login");

        return vec![
            format!("USER {} * * :{}\r\n", self.nickname, self.nickname),
            format!("NICK {}\r\n", self.nickname),
        ];
    }
}

impl Plugin for Pong {
    fn is_enabled(&self, message: &String) -> bool {
        return message.contains("PING");
    }

    fn perform(&self, message: &String) -> Vec<String> {
        log::info!("--> Executando Pong");

        let code: String = message
            .split(":")
            .collect::<Vec<&str>>()
            .last()
            .expect("BUMM")
            .to_string();

        return vec![format!("PONG :{}\r\n", code)];
    }
}

impl Plugin for Channel {
    fn is_enabled(&self, message: &String) -> bool {
        return message.contains("End of message of the da");
    }

    fn perform(&self, _message: &String) -> Vec<String> {
        log::info!("--> Executando Channel");
        return vec![format!("JOIN {}\r\n", self.channel)];
    }
}

impl Plugin for Logger {
    fn is_enabled(&self, _message: &String) -> bool {
        return true;
    }

    fn perform(&self, message: &String) -> Vec<String> {
        print!("<-- {}", message);
        return vec![];
    }
}

fn extract_metadata(message: &String) -> Option<regex::Captures<'_>> {
    let pattern = r"^:(?<nick>\w+)!(?<name>\w+)@(?<server>\w+.+) JOIN :#(?<channel>\w+)";
    let regex = Regex::new(pattern).unwrap();
    return regex.captures(message);
}

impl Plugin for Hello {
    fn is_enabled(&self, message: &String) -> bool {
        return message.contains(" JOIN :");
    }

    fn perform(&self, message: &String) -> Vec<String> {
        let metadata = extract_metadata(message).unwrap();
        let response = format!(
            "PRIVMSG #{} :{}: Iaaeee tru!\r\n",
            &metadata["channel"], &metadata["nick"]
        );

        return vec![response];
    }
}
