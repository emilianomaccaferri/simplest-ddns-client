use crate::client::error::StrategyError;
use serde::Deserialize;
use std::collections::HashMap;

use super::Strategy;

#[derive(Deserialize)]
struct ReqResponse {
    result: Vec<Dns>,
}

#[derive(Deserialize)]
struct Dns {
    id: String,
    name: String,
    r#type: String,
    ttl: i32,
}

#[derive(Deserialize)]
pub struct CloudFlareConfig {
    apikey: String,
    zoneid: String,
    hostname: String,
}

pub struct CloudFlareStrategy {
    pub config: CloudFlareConfig,
}

impl CloudFlareStrategy {
    pub fn new(config: Option<CloudFlareConfig>) -> Self {
        if config.is_none() {
            panic!("cloudflare config cannot be None!")
        }
        CloudFlareStrategy {
            config: config.unwrap(),
        }
    }
}

impl Strategy for CloudFlareStrategy {
    fn query(
        &self,
        client: &reqwest::blocking::Client,
        address: &str,
    ) -> Result<(), StrategyError> {
        let mut dns_info = Dns {
            id: String::new(),
            name: String::new(),
            r#type: String::new(),
            ttl: 1,
        };

        let mut url = format!(
            "https://api.cloudflare.com/client/v4/zones/{}/dns_records/",
            self.config.zoneid
        );

        let results = client
            .get(url.clone())
            .header("Authorization", "Bearer ".to_owned() + &self.config.apikey)
            .header("Content-Type", "application/json")
            .send()
            .unwrap()
            .json::<ReqResponse>()
            .unwrap();

        for result in results.result {
            if result.name == self.config.hostname {
                dns_info = result;
                break;
            }
        }
        if dns_info.id.is_empty() {
            return Err(StrategyError::HostnameError);
        }

        let mut map = HashMap::new();
        map.insert("type", dns_info.r#type);
        map.insert("name", self.config.hostname.clone());
        map.insert("content", address.to_string());
        map.insert("ttl", dns_info.ttl.to_string());

        url = format!("{}/{}", url, dns_info.id);

        match client
            .put(url)
            .header("Authorization", "Bearer ".to_owned() + &self.config.apikey)
            .header("Content-Type", "application/json")
            .json(&map)
            .send()
        {
            Err(_) => Err(StrategyError::NetworkError),
            Ok(stuff) => {
                println!(
                    "CloudFlare replied with: {:?}",
                    stuff
                        .text()
                        .unwrap_or_else(|_| "error while parsing response text".to_string())
                );
                Ok(())
            }
        }
    }
}
