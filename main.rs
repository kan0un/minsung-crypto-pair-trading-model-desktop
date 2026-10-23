use std::fs;
use std::io::Read;
use serde_json::Value;

#[derive(Debug)]
struct Config {
    items: Vec<String>,
}

fn load_config(path: &str) -> Config {
    let mut file = fs::File::open(path).expect("Failed to open config");
    let mut content = String::new();
    file.read_to_string(&mut content).expect("Failed to read config");
    let v: Value = serde_json::from_str(&content).expect("Failed to parse config");
    let items = v["items"].as_array().unwrap_or(&vec![])
        .iter()
        .filter_map(|x| x.as_str().map(|s| s.to_string()))
        .collect();
    Config { items }
}

fn main() {
    let config = load_config("config.json");
    println!("Loaded {} items", config.items.len());
}
