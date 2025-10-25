use std::collections::HashMap;

use regex::Regex;

pub fn regex_to_map(pattern: &str, payload: &String) -> HashMap<String, String> {
    let regex = Regex::new(pattern).expect("Problems trying to initialize Regex pattern");
    let mut results: HashMap<String, String> = HashMap::new();

    for caps in regex.captures_iter(payload) {
        for name in regex.capture_names() {
            if let Some(name_str) = name {
                if let Some(matched_value) = caps.name(name_str) {
                    results.insert(name_str.to_string(), matched_value.as_str().to_string());
                }
            }
        }
    }

    return results;
}

pub fn channel_message(metadata: HashMap<String, String>, message: &str) -> String {
    return format!(
        "PRIVMSG #{} - {}\r\n",
        metadata.get("channel").unwrap(),
        message,
    );
}

pub fn channel_user_message(metadata: HashMap<String, String>, message: &str) -> String {
    return format!(
        "PRIVMSG #{} {}: {}\r\n",
        metadata.get("channel").unwrap(),
        metadata.get("user").unwrap(),
        message,
    );
}
