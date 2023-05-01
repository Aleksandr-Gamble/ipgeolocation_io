//! [ipgeolocation.io](https://ipgeolocation.io/) contains a service to lookup the geographic location of IP addresses. The structs in this module correspond to the responses from that service.  
//! See [https://ipgeolocation.io/documentation/ip-geolocation-api.html](https://ipgeolocation.io/documentation/ip-geolocation-api.html)

use serde::{Serialize, Deserialize};


/// This struct contains information about a currency. 
#[derive(Serialize, Deserialize, Debug)]
pub struct Currency {
    /// The code for this currency, i.e. "USD"
    pub code: String,  // i.e. USD",
    /// The name for this currency, i.e. "Dollar"
    pub name: String,
    /// The symbol for this currency, i.e. "$"
    pub symbol: String, 
}


/// This struct describes a timezone 
#[derive(Serialize, Deserialize, Debug)]
pub struct TimeZone {
    /// A common name for the time zone based on geography, i.e. "America/Los_Angeles"
    pub name: String,  
    /// The offset from UTC
    pub offset: i8,
    /// Current timestamp in the specified timezone when the request was made 
    pub current_time: String,
    /// Current unix timestamp when the request was made 
    pub current_time_unix: f64,
    /// true for daylight savings time
    pub is_dst: bool,   
    /// 0 or 1 for daylight savings 
    pub dst_savings: u8,
}


/// This struct gives a description of an ip address along with its geographic location and
/// internet service provider (ISP). 
#[derive(Serialize, Deserialize, Debug)]
pub struct IpAddress {
    /// The string representation of the ip address, i.e. "8.8.8.8"
    pub ip: String, 
    /// The hostname for the string, if known, i.e. "dns.google"
    pub hostname: Option<String>,
    /// The continent code for the geographic location, i.e. "NA"
    pub continent_code: String, 
    /// The name of the continent, i.e. "North America"
    pub continent_name: String, 
    /// The two-letter country code, i.e. "US"
    pub country_code2: String,
    /// Rhe three-letter country code, i.e. "USA"
    pub country_code3: String,
    /// The country name, i.e. "United States"
    pub country_name: String,
    /// The capital of the associated country, i.e. "Washington D.C."
    pub country_capital: String,  
    /// State or province, i.e. "California"
    pub state_prov: String, 
    /// District within the state or province, i.e. "Santa Clara"
    pub district: String,  
    /// Geographic city, i.e. "Mountain View"
    pub city: String,
    /// zipcode, i.e. "94043-1351"
    pub zipcode: String,
    /// latitude for the city, i.e."37.42240" 
    pub latitude: String,
    /// longitude for the city, i.e. "-122.08421"
    pub longitude: String,
    /// true if the country is a member of the European Union 
    pub is_eu: bool,
    /// calling code prefix for the country, i.e. "+1"
    pub calling_code: String,  
    /// internet top-level-domain for the country 
    pub country_tld: String,
    /// list of common languages for the country, i.e. "en-US,es-US,haw,fr"
    pub languages: String,
    /// Link to the png of the flag for this country 
    pub country_flag: String,
    /// geographic id, i.e. "6301403"
    pub geoname_id: String,
    /// Internet service provider associated with the IP, i.e. "Google LLC"
    pub isp: String,
    /// connection type 
    pub connection_type: String,
    /// organization, if known, i.e. "Google LLC"
    pub organization: String,
    /// i.e. "AS15169"
    pub asn: Option<String>,
    pub currency: Currency,
    pub time_zone: TimeZone,
}

