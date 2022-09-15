mod client;
use std::fs;
use client::{ Config, Client };

fn main() {
    
    let config: String = fs::read_to_string("/etc/simplest-ovh-ddns-client/config.toml").unwrap();
    let parsed_config: Config = toml::from_str(config.as_str()).unwrap();

    println!("client started");
    println!("hostname: {}", parsed_config.ovh_hostname);

    Client::new(parsed_config).run();

}
