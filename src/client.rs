//! This module introduces the IpGeoClient. All it needs to be instantiated is an api key. Then you
//! can make asynchronous calls using the .parse_ip() and .parse_ua() methods.  


use reqwest::{get, Client, Error};
use crate::{ip, ua};


/// The IpGeoClient struct is the client. Just call .new() with an api key:
/// # Examples 
/// ```
/// use ipgeolocation_io::client::IpGeoClient;
/// let client = IpGeoClient::new("some_api_key");
/// ```
#[derive(Clone)]
pub struct IpGeoClient {
    api_key: String 
}


impl IpGeoClient {

    // Create a new client by providing the api key 
    pub fn new(api_key: &str) -> Self {
        let api_key = api_key.to_string();
        IpGeoClient{api_key}
    }

    // return the URL to get for parsing an ip address
    fn ipgeo_url(&self, ip_address: &str) -> String {
        let url = format!("https://api.ipgeolocation.io/ipgeo?apiKey={}&ip={}", &self.api_key, &ip_address);
        url 
    }

    // return the URL to get for parsing a user agent 
    fn uaparse_url(&self) -> String {
        let url = format!("https://api.ipgeolocation.io/user-agent?apiKey={}", &self.api_key);
        url 
    }


    pub async fn parse_ip(&self, ip_address: &str) -> Result<ip::IpAddress, Error> {
        let url = self.ipgeo_url(ip_address);
        let resp = get(&url)
            .await?
            .json::<ip::IpAddress>()
            .await?;
        Ok(resp)
    }

    pub async fn parse_ua(&self, user_agent: &str) -> Result<ua::UserAgent, Error> {
        let url = self.uaparse_url();
        let payload = ua::ReqPayload::new(user_agent);
        let client = Client::new();
        let resp = client.post(&url)
            .json(&payload)
            .send()
            .await?
            .json::<ua::UserAgent>()
            .await?;
        Ok(resp)
    }



}
