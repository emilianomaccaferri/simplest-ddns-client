use crate::client::error::StrategyError;
use serde::Deserialize;

use super::Strategy;

#[derive(Deserialize)]
pub struct DuckDnsConfig {
    token: String,
    subdomain: String,
}

pub struct DuckDnsStrategy {
    pub config: DuckDnsConfig,
}

impl DuckDnsStrategy {
    pub fn new(config: Option<DuckDnsConfig>) -> Self {
        if config.is_none() {
            panic!("duckdns config cannot be None!")
        }
        DuckDnsStrategy {
            config: config.unwrap(),
        }
    }
}

impl Strategy for DuckDnsStrategy {
    fn query(
        &self,
        client: &reqwest::blocking::Client,
        address: &str,
    ) -> Result<(), StrategyError> {
        let url = format!(
            "https://www.duckdns.org/update/{}/{}/{}",
            self.config.subdomain, self.config.token, address
        );

        match client
            .get(url)
            .send()
        {
            Err(_) => Err(StrategyError::NetworkError),
            Ok(stuff) => {
                println!(
                    "DuckDNS replied with: {:?}",
                    stuff
                        .text()
                        .unwrap_or_else(|_| "error while parsing response text".to_string())
                );
                Ok(())
            }
        }
    }
}
