//! Client interfaces for Streaming Availability API
//! See https://rapidapi.com/movie-of-the-night-movie-of-the-night-default/api/streaming-availability/

// TODO commonize with mdblist API client

use crate::cache::RequestCache;
use anyhow::Context;
use http::HeaderValue;
use serde::Deserialize;
use std::collections::BTreeMap;
use std::sync::Arc;

const API_HOST: &str = "streaming-availability.p.rapidapi.com";

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
                .context("invalid rapidapi API key")?,
        );

        let reqwest = reqwest::ClientBuilder::new()
            .default_headers(headers)
            .build()
            .context("initializing reqwest client")?;

        Ok(Client { reqwest, request_cache })
    }

    pub async fn lookup(
        &self,
        imdb_id: &str,
    ) -> Result<Availability, anyhow::Error> {
        let mut url =
            reqwest::Url::parse(&format!("https://{}/get/basic", API_HOST))
                .unwrap();
        url.query_pairs_mut()
            .append_pair("output_language", "en")
            .append_pair("country", "us")
            .append_pair("imdb_id", imdb_id);

        let url_str = url.as_str().to_string();
        let response = self.request_cache.request(&self.reqwest, url).await?;
        let result: Availability = serde_json::from_str(&response.body)
            .with_context(|| {
                format!("deserializing response for {:?}", url_str)
            })?;
        Ok(result)
    }
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Availability {
    #[serde(rename = "imdbID")]
    imdb_id: String,
    #[serde(rename = "tmdbID")]
    tmdb_id: String,
    imdb_rating: u16,
    imdb_vote_count: u64,
    tmdb_rating: u16,
    original_title: String,
    genres: Vec<u16>,
    countries: Vec<String>,
    year: u16,
    runtime: u16,
    cast: Vec<String>,
    significants: Vec<String>,
    title: String,
    overview: String,
    tagline: String,
    streaming_info: BTreeMap<String, BTreeMap<String, Streaming>>,
}

impl Availability {
    pub fn services(&self) -> Vec<String> {
        self.streaming_info.keys().cloned().collect()
    }
}

#[derive(Clone, Debug, Deserialize)]
struct Streaming {
    link: String,
    added: u64,   // TODO looks like a Unix timestamp
    leaving: u64, // TODO looks like a Unix timestamp
}
