pub mod xmltv {
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
        //#[serde(rename = "display-name")]
        //pub full_name: String,
        //#[serde(rename = "display-name")]
        //pub call_sign: String,
        //#[serde(rename = "display-name")]
        //pub channel_number: u16,
        #[serde(rename = "display-name")]
        pub display_names: [String;3],
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
        pub start: String,   // XXX
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
}
