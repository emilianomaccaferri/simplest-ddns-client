use super::error::StrategyError;

pub mod cloudflare;
pub mod ovh;

pub trait Strategy {
    fn query(&self, client: &reqwest::blocking::Client, address: &str)
        -> Result<(), StrategyError>;
}
