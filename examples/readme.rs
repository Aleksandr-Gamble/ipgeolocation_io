// This file is the example provided in README.md

use std::env;
use ipgeolocation_io::client::IpGeoClient;

#[tokio::main]
async fn main(){
    // Instantiate a client simply by providing an API key:
    let api_key = env::var("IPGEOLOCATION_IO_KEY").unwrap();
    let client = IpGeoClient::new(&api_key);

    // Look up geographic location, internet service provider, etc. for an ip address:
    let ip_address = client.parse_ip("8.8.8.8").await.unwrap();
    println!("{:?}", &ip_address);

    // Parse a user agent string to
    let ua_str = "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko)
    Chrome/95.0.4638.69 Safari/537.36";
    let user_agent = client.parse_ua(&ua_str).await.unwrap();
    println!("{:?}", &user_agent);
}

