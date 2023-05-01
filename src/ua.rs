//! ipgeolocation.io contains a service to parse user agents
//! The structs in this module correspond to the responses from that service 
//! See [https://ipgeolocation.io/documentation/user-agent-api.html](https://ipgeolocation.io/documentation/user-agent-api.html)  
//! **NOTE**: The custom user_agent parsing method is only available with paid subscriptions.  
//! If you have the developer (free) subscription, you will get this error:                                                       
//! *"Custom User-Agent lookup is not supported on your free subscription. This feature is
//! available to all paid subscriptions only."*
//!

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



/// When parsing user_agents, this struct represents a device.
#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Device {
    /// The name of the device, i.e. "Apple Macintosh"
    pub name: String,
    /// The type of device, i.e. "Desktop"
    pub r#type: String,
    /// The brand of device, i.e. "Apple"
    pub brand: String,
    /// The type of CPU, i.e. "Intel" but will be the None variant for some web crawlers 
    pub CPU: Option<String>,  
}


/// When parsing user agents, this struct represents a browser engine. 
#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Engine {
    /// The name of the engine, i.e. "AppleWebKit"
    pub name: String,
    /// The type of engine, typically "Browser"
    pub r#type: String, 
    /// The version, i.e. "601.3.9"
    pub version: String,
    /// The major version, i.e. "601"
    pub versionMajor: String,
    /// build, if specified, often "Unknown" 
    pub build: String,
}


/// When parsing user agents, this struct represents the operating system.
#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct OperatingSystem {
    /// The name of the operating system, i.e. "Mac OS X"
    pub name: String,
    /// The type of system the OS is intended for, i.e. "Desktop"
    pub r#type: String, 
    /// OS version, i.e. "10.11.2"
    pub version: String,
    /// OS major version, i.e. "10"
    pub versionMajor: String,
}


/// This struct represents the body of the response from the user agent parser.
#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct UserAgent {
    /// The full user agent string: tyically something like   
    /// "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_11_2) AppleWebKit/601.3.9 (KHTML, like Gecko) Version/9.0.2 Safari/601.3.9"
    pub userAgentString: String,
    /// The name of the agent, i.e. "Safari"
    pub name: String,
    /// The type of agent, often "Browser"
    pub r#type: String, 
    /// The version, i.e. "9.0.2"
    pub version: String,
    /// The major version, i.e. "9"
    pub versionMajor: String,
    pub device: Device,
    pub engine: Engine,
    pub operatingSystem: OperatingSystem,
}




