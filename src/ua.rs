//! ipgeolocation.io contains a service to parse user agents
//! The structs in this module correspond to the responses from that service 
//! See [https://ipgeolocation.io/documentation/user-agent-api.html](https://ipgeolocation.io/documentation/user-agent-api.html)


use serde::{Serialize, Deserialize};

/// This struct needs to be passed as the payload to get parse a user_agent
/// Different than the device making the request 
#[allow(non_snake_case)]
#[derive(Serialize)]
pub struct ReqPayload {
    uaString: String,
}

#[allow(non_snake_case)]
impl ReqPayload {
    pub fn new(uaString: &str) -> Self {
        let uaString = uaString.to_string();
        ReqPayload{uaString}
    }
}



/// when parsing user_agents, this struct represents a device 
#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Device {
    pub name: String,  // i.e.  "Apple Macintosh",
    pub r#type: String,  // i.e.  "Desktop",
    pub brand: String,  // i.e.  "Apple",
    pub CPU: Option<String>,  // i.e.  "Intel", will be None for some web crawlers
}


/// when parsing user agents, this struct represents a browser engine 
#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Engine {
    pub name: String,  // i.e.  "AppleWebKit",
    pub r#type: String,  // i.e.  "Browser",
    pub version: String,  // i.e.  "601.3.9",
    pub versionMajor: String,  // i.e.  "601",
    pub build: String,  // i.e.  "Unknown"
}


/// when parsing user agents, this struct represents the operating system 
#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct OperatingSystem {
    pub name: String,  // i.e.  "Mac OS X",
    pub r#type: String,  // i.e.  "Desktop",
    pub version: String,  // i.e.  "10.11.2",
    pub versionMajor: String,  // i.e.  "10"
}


/// This struct represents the body of the response from the user agent parser
#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct UserAgent {
    pub userAgentString: String,  // i.e.  "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_11_2) AppleWebKit/601.3.9 (KHTML, like Gecko) Version/9.0.2 Safari/601.3.9",
    pub name: String,  // i.e.  "Safari",
    pub r#type: String,  // i.e.  "Browser",
    pub version: String,  // i.e.  "9.0.2",
    pub versionMajor: String,  // i.e.  "9",
    pub device: Device,
    pub engine: Engine,
    pub operatingSystem: OperatingSystem,
}
    