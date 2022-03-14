use anyhow::Context;
use serde::Deserialize;
use std::io::{BufReader, Read};

#[derive(Debug, Deserialize)]
pub struct Tv {
    pub date: String, // XXX
    #[serde(rename = "channel")]
    pub channels: Vec<Channel>,
    #[serde(rename = "programme")]
    pub programs: Vec<Program>,
}

#[derive(Debug, Deserialize)]
pub struct Channel {
    pub id: String,
    #[serde(rename = "display-name")]
    pub display_names: [String; 3],
}

impl Channel {
    pub fn full_name(&self) -> &str {
        &self.display_names[0]
    }
    pub fn callsign(&self) -> &str {
        &self.display_names[1]
    }
    pub fn channel_number(&self) -> &str {
        &self.display_names[2]
    }
}

#[derive(Debug, Deserialize)]
pub struct Program {
    #[serde(deserialize_with = "deserialize_xmltv_datetime")]
    pub start: chrono::DateTime<chrono::Local>,
    pub stop: String,    // XXX
    pub channel: String, // XXX
    pub title: String,
    pub category: String,
}

impl Tv {
    pub fn from_reader<R: Read>(r: R) -> Result<Tv, anyhow::Error> {
        let bufread = BufReader::new(r);
        quick_xml::de::from_reader(bufread).context("parsing xmltv input")
    }
}

pub fn deserialize_xmltv_datetime<'de, D>(
    deserializer: D,
) -> Result<chrono::DateTime<chrono::Local>, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    // Example timestamp: "20220326223000 +0000"
    const FORMAT: &str = "%Y%m%d%H%M%S %z";
    let s = String::deserialize(deserializer)?;
    Ok(chrono::DateTime::parse_from_str(&s, FORMAT)
        .map_err(serde::de::Error::custom)?
        .with_timezone(&chrono::Local))
}
