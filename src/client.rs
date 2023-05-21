//! This module introduces the IpGeoClient. All it needs to be instantiated is an api key. Then you
//! can make asynchronous calls using the .parse_ip() and .parse_ua() methods.  
//! 
//! # Examples
//!
//! ```
//! use std::env;
//! use ipgeolocation_io::client::IpGeoClient;  
//! 
//! #[tokio::main]
//! async fn main(){
//!     let api_key = env::var("IPGEOLOCATION_IO_KEY").unwrap();
//!     let client = IpGeoClient::new(&api_key);
//!   
//!     let ip_address = client.parse_ip("8.8.8.8").await.unwrap();
//!     println!("{:?}", &ip_address);     
//!     
//!     let ua_str = "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko)
//!     Chrome/95.0.4638.69 Safari/537.36";
//!     let user_agent = client.parse_ua(&ua_str).await.unwrap();
//!     println!("{:?}", &user_agent);
//! }
//! ```
//!

use serde::{Serialize, Deserialize};
use reqwest::{get, Client, Error as ReqwestError};
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


    pub async fn parse_ip(&self, ip_address: &str) -> Result<ip::IpAddress, IpGeoError> {
        let url = self.ipgeo_url(ip_address);
        let resp = get(&url)
            .await?
            .json::<ResultIP>()
            .await?;
        match resp {
            ResultIP::Succ(ip_addy) => Ok(ip_addy),
            ResultIP::ErrMsg(err_msg) => Err(err_msg.into()),
        }
    }


    pub async fn parse_ua(&self, user_agent: &str) -> Result<ua::UserAgent, IpGeoError> {
        let url = self.uaparse_url();
        let payload = ua::ReqPayload::new(user_agent);
        let client = Client::new();
        let resp = client.post(&url)
            .json(&payload)
            .send()
            .await?
            .json::<ResultUA>()
            .await?;
        match resp {
            ResultUA::Succ(usr_agnt) => Ok(usr_agnt),
            ResultUA::ErrMsg(err_msg) => Err(err_msg.into()),
        }
    }

}


/// Most error messages returned by the API are of the format below. Two common ones are
/// 1) *"Custom User-Agent lookup is not supported on your free subscription. This feature is
/// available to all paid subscriptions only."*  
/// 2) *"Provided API key is not valid. Contact technical support for assistance at support@ipgeolocation.io"*  
#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorMessage {
    /// The two most common errors are  
    /// 1) *"Custom User-Agent lookup is not supported on your free subscription. This feature is
    /// available to all paid subscriptions only."*  
    /// 2) *"Provided API key is not valid. Contact technical support for assistance at support@ipgeolocation.io"*  
    pub message: String
}


impl std::error::Error for ErrorMessage {
    fn description(&self) -> &str {
        &self.message 
    }
}


impl std::fmt::Display for ErrorMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f,"{}",self.message )
    }
}



// This private struct deserializes a ua::UserAgent OR an ErrorMessage
#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
enum ResultUA {
    Succ(ua::UserAgent),
    ErrMsg(ErrorMessage),
}

// This private struct deserializes an ip::IpAddress OR an ErrorMessage
#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
enum ResultIP {
    Succ(ip::IpAddress),
    ErrMsg(ErrorMessage),
}


/// This error has enums for a generic HTTP error (from reqwest)
/// as well as an error message provided by the API  
#[derive(Debug)]
pub enum IpGeoError {
    HTTP(ReqwestError),
    Auth(ErrorMessage),
}

impl std::error::Error for IpGeoError {}

impl std::fmt::Display for IpGeoError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f,"{:?}", &self )
    }
}


impl From<ReqwestError> for IpGeoError {
    fn from(e: ReqwestError) -> Self {
        IpGeoError::HTTP(e)
    }
}

impl From<ErrorMessage> for IpGeoError {
    fn from(e: ErrorMessage) -> Self {
        IpGeoError::Auth(e)
    }
}

