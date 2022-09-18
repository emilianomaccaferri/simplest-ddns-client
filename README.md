# simplest-ddns-client
A very simple and minimal DDNS client that checks if your public IP has changed and updates a specified DNS zone.

# how to
* create a `config.toml` file in `/etc/simplest-ddns-client/`
* install Rust
* clone this repo
* change `provider` in `main.rs` to one of these [providers](#configurations-list)
* run `cargo build --release`
* binary will be available in `target/release/`

## the `config.toml` file
This file will contain all the information needed (generally authentication stuff) to update your DNS records.<br>
The first entry in this file is the initial address the client will base its queries from: `initial_ip`.<br>

## configurations list
- [OVH](#ovh-configuration)
- [CloudFlare](#cloudflare-configuration)

## OVH configuration
In the `[ovh]` section

```
user = "your-ovh-user" # the user you registered on ovh
password = "ovh-password" # the password you chose on ovh
hostname = "hostname.example.com" # the hostname you registered as dynamic on ovh
```
A complete `config.toml` file for OVH looks something like this:

```
initial_ip = "1.1.1.1" # the ip the client will use when started
[ovh]
user = "your-ovh-user" # the user you registered on ovh
password = "ovh-password" # the password you chose on ovh
hostname = "hostname.example.com" # the hostname you registered as dynamic on ovh
```

Refer to this guide for more: https://docs.ovh.com/gb/en/domains/hosting_dynhost/

## CloudFlare configuration
A complete `config.toml` file for CloudFlare looks something like this:

```
initial_ip = "1.1.1.1" # the ip the client will use when started
[cloudflare]
apikey = "your-apikey" # API Token generated from the User Profile 'API Tokens' page
zoneid = "zoneid" # domain's zone id
hostname = "hostname.example.com" # the hostname you want to update
```

Refer to [this](https://developers.cloudflare.com/fundamentals/get-started/basic-tasks/find-account-and-zone-ids/) guide to get `zoneid`.
