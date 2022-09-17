mod error;

use std::{thread, time::Duration};
use lazy_static::lazy_static;
use regex::Regex;
use serde::Deserialize;
use self::error::ClientError;

#[derive(Deserialize)]
pub struct Config {
    pub initial_ip: String,
    pub ovh_user: String,
    pub ovh_password: String,
    pub ovh_hostname: String
}

pub struct Client{
    address: String,
    user: String,
    password: String,
    hostname: String,
    client: reqwest::blocking::Client
}

impl Client {
    pub fn new(config: Config) -> Self {

        Client { 
            address: config.initial_ip, 
            user: config.ovh_user, 
            hostname: config.ovh_hostname,
            password: config.ovh_password,
            client: reqwest::blocking::Client::builder().timeout(Duration::from_secs(5)).build().unwrap() 
        }

    }

    pub fn run(self: &mut Self) -> ! { // -> ! = "never" type
        
        loop {
            
            // get WAN IP from remote thing
            match self.address_check() {
                Err(ClientError::InvalidPage) => eprintln!("invalid page"),
                Err(ClientError::NetworkError) => eprintln!("network error while trying to obtain wan ip"),
                Err(ClientError::NotOk(s)) => eprintln!("request failed with code {}", s),
                Ok(address) => self.handle(address)
            }
            
            thread::sleep(Duration::from_millis(10000));

        }
        
    }

    fn handle(self: &mut Self, address: Option<String>) -> () {

        // check if the IP needs to be changed

        if address.to_owned().is_none() {
            println!("invalid output");
            return;
        }

        if address.to_owned().unwrap().eq(&self.address) {
            println!("ip hasn't changed");
            return;
        }

        println!("ip changed from {} to {}, telling OVH...", self.address, address.to_owned().unwrap());

        self.address = address.to_owned().unwrap();
        let url = format!(
            "https://www.ovh.com/nic/update?system=dyndns&hostname={}&myip={}"
        , self.hostname, address.to_owned().unwrap());

        match self.client.get(url)
            .basic_auth(&self.user, Some(&self.password))
            .send() {
                
                Err(_) => eprintln!("network error while changing ip..."),
                Ok(stuff) => {
                    
                    println!("OVH replied with: {:?}", stuff.text().unwrap_or("error while parsing response text".to_string()));
                    
                } 
        
        }

    }

    fn address_check(self: &Self) -> Result<Option<String>, error::ClientError> {

        let req = match self.client.get("http://checkip.dyndns.com").send() {
            Ok(r) => r,
            Err(_) => return Err(ClientError::NetworkError)
        };
        
        if req.status().as_str() != "200" {
            return Err(ClientError::NotOk(req.status().to_string()))
        }

        let body = match req.text() {
            Ok(t) => t,
            Err(_) => return Err(ClientError::InvalidPage)
        };
        
        lazy_static!{
            static ref RE: Regex = Regex::new("<html><head><title>Current IP Check</title></head><body>Current IP Address: (.*)</body></html>").unwrap();
        }
        let captures = RE.captures(body.as_str());

        if let Some(unwrapped) = captures {

            if unwrapped.len() > 1 {
                
                return Ok(
                    Some(
                        String::from(unwrapped.get(1).map(| m | m.as_str()).unwrap())
                    )
                );
                
            }
            
            Ok(None)

        } else {

            Ok(None)

        }

    }
}
