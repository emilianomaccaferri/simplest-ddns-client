mod client;

use client::{Client, GlobalConfig};
use std::fs;

fn main() {
    let config: String = fs::read_to_string("/etc/simplest-ddns-client/config.toml").unwrap();
    let parsed_config: GlobalConfig = toml::from_str(config.as_str()).unwrap();

    println!("client started");
    println!("provider: {}", parsed_config.provider);
    println!("initial ip: {}", parsed_config.initial_ip);

    Client::new(parsed_config).run();
}
