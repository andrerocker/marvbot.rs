use regex::Regex;

pub trait Plugin {
    fn is_enabled(&self, message: &String) -> bool;
    fn perform(&self, message: &String) -> Vec<String>;
}

pub struct Logger {}
pub struct Login {}
pub struct Pong {}
pub struct Channel {}
pub struct Hello {}

impl Plugin for Login {
    fn is_enabled(&self, message: &String) -> bool {
        return message.contains("Could not resolve your hostname");
    }

    fn perform(&self, _: &String) -> Vec<String> {
        println!("--> Executando Login");

        return vec![
            "USER marv * * :Marv\r\n".to_string(),
            "NICK marv\r\n".to_string(),
        ];
    }
}

impl Plugin for Pong {
    fn is_enabled(&self, message: &String) -> bool {
        return message.contains("PING");
    }

    fn perform(&self, message: &String) -> Vec<String> {
        println!("--> Executando Pong");

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
        println!("--> Executando Channel");
        return vec![String::from("JOIN #acme\r\n")];
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
    let regex = Regex::new(r"^:(?<nick>\w+)!(?<name>\w+)@(?<server>\w+.+) JOIN :#(?<channel>\w+)").unwrap();
    return regex.captures(message);
}

impl Plugin for Hello {
    fn is_enabled(&self, message: &String) -> bool {
        return message.contains(" JOIN :");
    }

    fn perform(&self, message: &String) -> Vec<String> {
        let metadata = extract_metadata(message).unwrap(); 
        let response = format!("PRIVMSG #{} :{}: Iaaeee tru!\r\n", &metadata["channel"], &metadata["nick"]);
        print!("--> {} - {:?}", response, metadata);

        return vec![response];
    }
}

#[cfg(test)]
mod plugins_test;