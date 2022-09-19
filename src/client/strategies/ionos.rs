use crate::client::error::StrategyError;
use serde::{Deserialize, Serialize};

use super::Strategy;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ReqResponse {
    update_url: String,
}

#[derive(Serialize)]
struct DynDns {
    domains: Vec<String>,
    description: String,
}

#[derive(Deserialize)]
pub struct IONOSConfig {
    prefix: String,
    secret: String,
    hostname: String,
}

pub struct IONOSStrategy {
    pub config: IONOSConfig,
}

impl IONOSStrategy {
    pub fn new(config: Option<IONOSConfig>) -> Self {
        if config.is_none() {
            panic!("ionos config cannot be None!")
        }
        IONOSStrategy {
            config: config.unwrap(),
        }
    }
}

impl Strategy for IONOSStrategy {
    fn query(
        &self,
        client: &reqwest::blocking::Client,
        _address: &str,
    ) -> Result<(), StrategyError> {
        let dns_info = DynDns {
            domains: vec![self.config.hostname.clone()],
            description: "DynamicDns made with simplest-ddns-client".to_string(),
        };

        let url = "https://api.hosting.ionos.com/dns/v1/dyndns";
        let xapikey = format!("{}.{}", &self.config.prefix, &self.config.secret);

        let results = client
            .post(url)
            .header("user-agent", "curl/7.84.0")
            .header("accept", "application/json")
            .header("X-API-Key", &xapikey)
            .header("Content-Type", "application/json")
            .json(&dns_info)
            .send()
            .unwrap()
            .json::<ReqResponse>()
            .unwrap();

        match client
            .get(results.update_url)
            .header("user-agent", "curl/7.84.0")
            .header("accept", "application/json")
            .header("X-API-Key", &xapikey)
            .send()
        {
            Err(_) => Err(StrategyError::NetworkError),
            Ok(stuff) => {
                println!(
                    "IONOS replied with: {:?}",
                    stuff
                        .text()
                        .unwrap_or_else(|_| "error while parsing response text".to_string())
                );
                Ok(())
            }
        }
    }
}
