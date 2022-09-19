# simplest-ddns-client
A very simple and minimal DDNS client that checks if your public IP has changed and updates a specified DNS zone.

# how to
* create a `config.toml` file in `/etc/simplest-ddns-client/`
* install Rust
* clone this repo
* change `provider` in `config.toml` to one of these [providers](#configurations-list)
* run `cargo build --release`
* binary will be available in `target/release/`

## the `config.toml` file
This file will contain all the information needed (generally authentication stuff) to update your DNS records.<br>
The first entry in this file is the initial address the client will base its queries from: `initial_ip`.<br>
There is an example file located at the root dir called `example_config.toml`.

## configurations list
* [OVH](#ovh-configuration)
* [CloudFlare](#cloudflare-configuration)
* [DuckDNS](#duckdns-configuration)
* [IONOS](#ionos-configuration)

## OVH configuration
A complete `config.toml` file for OVH looks something like this:

```
provider = "ovh"
initial_ip = "1.1.1.1"
[ovh]
user = "your-ovh-user" # the user you registered on ovh
password = "ovh-password" # the password you chose on ovh
hostname = "hostname.example.com" # the hostname you registered as dynamic on ovh
```

Refer to [this](https://docs.ovh.com/gb/en/domains/hosting_dynhost/) guide for more.

## CloudFlare configuration
A complete `config.toml` file for CloudFlare looks something like this:

```
provider = "cloudflare"
initial_ip = "1.1.1.1"
[cloudflare]
apikey = "your-apikey" # API Token generated from the User Profile 'API Tokens' page
zoneid = "zoneid" # domain's zone id
hostname = "hostname.example.com" # the hostname you want to update
```

Refer to [this](https://developers.cloudflare.com/fundamentals/get-started/basic-tasks/find-account-and-zone-ids/) guide to get `zoneid`.

## DuckDNS configuration
A complete `config.toml` file for DuckDNS looks something like this:

```
provider = "duckdns"
initial_ip = "1.1.1.1"
[duckdns]
token = "your-token" # the token duckdns gives to you
subdomain = "your-subdomain" # the subdomain you chose on duckdns
```

## IONOS configuration
```
provider = "ionos"
initial_ip = "1.1.1.1"
[ionos]
prefix = "your-public-prefix" # the public prefix IONOS gives to you
secret = "your-secret" # the secret key IONOS gives to you
hostname = "hostname.example.com" # the hostname you want to update
```

Refer to [this](https://developer.hosting.ionos.com/docs/getstarted) guide to get an apikey.
