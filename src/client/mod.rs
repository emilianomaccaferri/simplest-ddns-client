mod error;
mod strategies;

use self::{
    error::ClientError,
    strategies::{
        ovh::{OVHConfig, OVHStrategy},
        Strategy,
    },
};
use lazy_static::lazy_static;
use regex::Regex;
use serde::Deserialize;
use std::{thread, time::Duration};

#[derive(Deserialize)]
pub struct GlobalConfig {
    pub initial_ip: String,
    pub ovh: Option<OVHConfig>,
}

pub struct Client {
    address: String,
    client: reqwest::blocking::Client,
    strategy: Box<dyn Strategy>,
}

impl Client {
    pub fn new(config: GlobalConfig, strategy_name: &str) -> Self {
        let strategy: Box<dyn Strategy> = match strategy_name {
            "ovh" => Box::from(OVHStrategy::new(config.ovh)),
            _ => panic!("invalid strategy name!"),
        };

        Client {
            address: config.initial_ip,
            client: reqwest::blocking::Client::builder()
                .timeout(Duration::from_secs(5))
                .build()
                .unwrap(),
            strategy,
        }
    }

    pub fn run(&mut self) -> ! {
        // -> ! = "never" type

        loop {
            // get WAN IP from remote thing
            match self.address_check() {
                Err(ClientError::InvalidPage) => eprintln!("invalid page"),
                Err(ClientError::NetworkError) => {
                    eprintln!("network error while trying to obtain wan ip")
                }
                Err(ClientError::NotOk(s)) => eprintln!("request failed with code {}", s),
                Ok(address) => self.handle(&address),
            }

            thread::sleep(Duration::from_millis(10000));
        }
    }

    fn handle(&mut self, address: &Option<String>) {
        // check if the IP needs to be changed
        if let Some(ip) = address {
            if ip.eq(&self.address) {
                return;
            }
            println!(
                "address changed from {} to {}, telling provider...",
                self.address, ip
            );

            self.address = ip.clone();
            match self.strategy.query(&self.client, &self.address) {
                Err(err) => {
                    println!("DDNS request failed: {:?}", err)
                }
                Ok(()) => {
                    println!("DDNS request successful!")
                }
            }
        }
    }

    fn address_check(&self) -> Result<Option<String>, error::ClientError> {
        let req = match self.client.get("http://checkip.dyndns.com").send() {
            Ok(r) => r,
            Err(_) => return Err(ClientError::NetworkError),
        };

        if req.status().as_str() != "200" {
            return Err(ClientError::NotOk(req.status().to_string()));
        }

        let body = match req.text() {
            Ok(t) => t,
            Err(_) => return Err(ClientError::InvalidPage),
        };

        lazy_static! {
            static ref RE: Regex = Regex::new("<html><head><title>Current IP Check</title></head><body>Current IP Address: (.*)</body></html>").unwrap();
        }
        let captures = RE.captures(body.as_str());

        if let Some(unwrapped) = captures {
            if unwrapped.len() > 1 {
                return Ok(Some(String::from(
                    unwrapped.get(1).map(|m| m.as_str()).unwrap(),
                )));
            }

            Ok(None)
        } else {
            Ok(None)
        }
    }
}
