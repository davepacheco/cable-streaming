//! Client interfaces for MDBList API
//! See https://rapidapi.com/linaspurinis/api/mdblist/

use crate::cache::RequestCache;
use anyhow::Context;
use http::HeaderValue;
use serde::Deserialize;
use std::sync::Arc;

const API_HOST: &str = "mdblist.p.rapidapi.com";

pub struct Client {
    reqwest: reqwest::Client,
    request_cache: Arc<RequestCache>,
}

impl Client {
    pub fn new(
        api_key: &str,
        request_cache: Arc<RequestCache>,
    ) -> Result<Client, anyhow::Error> {
        let mut headers = http::HeaderMap::new();
        headers.insert("x-rapidapi-host", HeaderValue::from_static(API_HOST));
        headers.insert(
            "x-rapidapi-key",
            HeaderValue::from_str(api_key)
                .context("invalid mdblist API key")?,
        );

        let reqwest = reqwest::ClientBuilder::new()
            .default_headers(headers)
            .build()
            .context("initializing reqwest client")?;

        Ok(Client { reqwest, request_cache })
    }

    pub async fn title_lookup(
        &self,
        title: &str,
    ) -> Result<Vec<Match>, anyhow::Error> {
        let mut url =
            reqwest::Url::parse(&format!("https://{}/", API_HOST)).unwrap();
        // XXX escaping
        url.query_pairs_mut()
            .append_pair("s", &format!("\"{}\"", title))
            .append_pair("m", "movie");

        let url_str = url.as_str().to_string();
        let response = self.request_cache.request(&self.reqwest, url).await?;
        let result: SearchResult = serde_json::from_str(&response.body)
            .with_context(|| {
                format!("deserializing response for {:?}", url_str)
            })?;
        Ok(result.search)
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct Match {
    pub id: String,
    pub title: String,
    pub year: Option<u16>,
    pub score: i16,
    #[serde(rename = "type")]
    type_name: String,
    pub imdbid: Option<String>,
    pub traktid: u64,
}

#[derive(Debug, Deserialize)]
struct SearchResult {
    search: Vec<Match>,
    total: usize,
    response: bool,
}

pub fn prune(results: &[Match]) -> Vec<Match> {
    let mut rv: Vec<Match> = results
        .into_iter()
        .filter_map(|m| {
            (m.year.is_some() && m.imdbid.is_some() && m.score > 0)
                .then(|| m.clone())
        })
        .collect();
    rv.sort_by(|m1, m2| m2.score.cmp(&m1.score));
    rv
}
