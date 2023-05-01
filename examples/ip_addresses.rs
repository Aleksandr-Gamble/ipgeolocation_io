use std::env;
use ipgeolocation_io::client::IpGeoClient;

#[tokio::main]
async fn main(){
    let api_key = env::var("IPGEOLOCATION_IO_KEY").unwrap();
    let client = IpGeoClient::new(&api_key);
    loop {
        let mut input = String::new();
        println!("\n\nEnter an IP address: ");
        let _ = std::io::stdin().read_line(&mut input).unwrap();
        let resp = client.parse_ip(&input).await.unwrap();
        println!("{:?}", &resp);
    }
}
