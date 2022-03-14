use serde::Serialize;
use serde::Deserialize;

pub mod mdblist;
pub mod streaming_availability;
pub mod xmltv;

#[derive(Debug, Deserialize, Serialize)]
pub struct Credentials {
    pub xml_tv_listings_api_key: String,
    pub rapidapi_key: String,
}
