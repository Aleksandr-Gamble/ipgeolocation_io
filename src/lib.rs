//! IpGeoLocation.io  
//!   
//! This crate contains a simple, open-source client for
//! [ipgeolocation.io](https://ipgeolocation.io/)  
//!   
//! The client is built within the [tokio](https://tokio.rs/) asyncrhonous runtime.
//! Modules are included for two main tasks:  
//! - determining the geographic location of an IP address  
//! - parsing a user agent string  
//!   
//! All you have to do is provide an API key (and the developer tier is free!).
//!

pub mod client;
pub mod ip;
pub mod ua;
