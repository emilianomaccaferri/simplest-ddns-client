# simplest-ovh-ddns-client
a very simple and minimal client that checks if your public IP has changed and updates it on OVH

# how to
* create a `config.toml` file in `/etc/simplest-ovh-ddns-client`
* install Rust
* clone this repo
* `cargo build --release`
* binary will be available in `target/release`
## the `config.yml` file
```
initial_ip = "1.1.1.1" # the ip the client will use when started 
ovh_user = "your-ovh-user" # the user you registered on ovh
ovh_password = "ovh-password" # the password you chose on ovh
ovh_hostname = "hostname.example.com" # the hostname you registered as dynamic on ovh 
```
Refer to this guide for more: https://docs.ovh.com/gb/en/domains/hosting_dynhost/
