use super::error::StrategyError;

pub mod ovh;
pub mod cloudflare;
pub mod duckdns;

pub trait Strategy {
    fn query(&self, client: &reqwest::blocking::Client, address: &str)
        -> Result<(), StrategyError>;
}
