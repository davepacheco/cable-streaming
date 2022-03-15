use anyhow::Context;
use serde::Deserialize;
use serde::Serialize;
use std::path::Path;

#[derive(Debug, Deserialize, Serialize)]
pub struct Credentials {
    pub xml_tv_listings_api_key: String,
    pub rapidapi_key: String,
}

impl Credentials {
    pub fn from_file(creds_path: &Path) -> Result<Credentials, anyhow::Error> {
        let creds_file = std::fs::read_to_string(creds_path)
            .with_context(|| format!("open {:?}", creds_path.display()))?;
        toml::from_str(&creds_file)
            .with_context(|| format!("parse {:?}", creds_path.display()))
    }

    pub fn from_default_file() -> Result<Credentials, anyhow::Error> {
        Credentials::from_file(Path::new("creds.toml"))
    }
}
