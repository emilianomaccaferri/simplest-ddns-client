# simplest-ddns-client
A very simple and minimal DDNS client that checks if your public IP has changed and updates a specified DNS zone.

# how to
* create a `config.toml` file in `/etc/simplest-ddns-client/`
* install Rust
* clone this repo
* run `cargo build --release`
* binary will be available in `target/release/`

## the `config.toml` file
This file will contain all the information needed (generally authentication stuff) to update your DNS records.<br>
The first entry in this file is the initial address the client will base its queries from: `initial_ip`<br>
**NOTE: OVH is the only provider currently supported by this client, but I'm planning to add more soon!<br>**

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
ovh_user = "your-ovh-user" # the user you registered on ovh
ovh_password = "ovh-password" # the password you chose on ovh
ovh_hostname = "hostname.example.com" # the hostname you registered as dynamic on ovh
```

Refer to this guide for more: https://docs.ovh.com/gb/en/domains/hosting_dynhost/
