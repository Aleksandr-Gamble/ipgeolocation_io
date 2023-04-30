use std::env;
use tokio;
use ipgeolocation_io::ipgeo::IpGeoClient;

#[tokio::main]
async fn main(){
    let api_key = env::var("IPGEOLOCATION_IO_KEY").unwrap();
    let client = IpGeoClient::new(&api_key);
    let resp = client.geoloc_ip("8.8.8.8").await.unwrap();
    println!("{:?}", &resp);
}
