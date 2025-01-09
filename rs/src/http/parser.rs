use std::collections::HashMap;

pub fn parse_headers_to_hashmap(headers: String) -> HashMap<String, String> {
    let mut header_map = HashMap::new();
    for line in headers.lines() {
        if let Some((key, value)) = line.split_once(": ") {
            header_map.insert(key.to_string(), value.to_string());
        }
        else if let Some((key, value)) = line.split_once("=") {
            header_map.insert(key.to_string(), value.to_string());
        }
    }
    header_map
}

pub fn print_headers(hearder: HashMap<String, String>) {
    for (key, val) in hearder {
        println!("{}: {}", key, val);
    }
}

