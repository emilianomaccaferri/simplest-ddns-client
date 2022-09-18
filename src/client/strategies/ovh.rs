use crate::client::error::StrategyError;
use serde::Deserialize;

use super::Strategy;

#[derive(Deserialize)]
pub struct OVHConfig {
    user: String,
    password: String,
    hostname: String,
}

pub struct OVHStrategy {
    pub config: OVHConfig,
}

impl OVHStrategy {
    pub fn new(config: Option<OVHConfig>) -> Self {
        if config.is_none() {
            panic!("ovh config cannot be None!")
        }
        OVHStrategy {
            config: config.unwrap(),
        }
    }
}

impl Strategy for OVHStrategy {
    fn query(
        &self,
        client: &reqwest::blocking::Client,
        address: &str,
    ) -> Result<(), StrategyError> {
        let url = format!(
            "https://www.ovh.com/nic/update?system=dyndns&hostname={}&myip={}",
            self.config.hostname, address
        );

        match client
            .get(url)
            .basic_auth(&self.config.user, Some(&self.config.password))
            .send()
        {
            Err(_) => Err(StrategyError::NetworkError),
            Ok(stuff) => {
                println!(
                    "OVH replied with: {:?}",
                    stuff
                        .text()
                        .unwrap_or_else(|_| "error while parsing response text".to_string())
                );
                Ok(())
            }
        }
    }
}
