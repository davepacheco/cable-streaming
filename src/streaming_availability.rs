//! Client interfaces for Streaming Availability API
//! See https://rapidapi.com/movie-of-the-night-movie-of-the-night-default/api/streaming-availability/

// TODO commonize with mdblist API client

use anyhow::bail;
use anyhow::Context;
use http::HeaderValue;
use serde::Deserialize;

const API_HOST: &str = "streaming-availability.p.rapidapi.com";

pub struct Client {
    reqwest: reqwest::Client,
}

impl Client {
    pub fn new(api_key: &str) -> Result<Client, anyhow::Error> {
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

        Ok(Client { reqwest })
    }

    pub async fn lookup(
        &self,
        imdb_id: &str,
    ) -> Result<Availability, anyhow::Error> {
        let mut url =
            reqwest::Url::parse(&format!("https://{}/get/basic", API_HOST))
                .unwrap();
        // XXX escaping
        url.query_pairs_mut()
            .append_pair("output_language", "en")
            .append_pair("country", "us")
            .append_pair("imdb_id", imdb_id);
        let request =
            self.reqwest.get(url).build().context("failed to build request")?;
        let response =
            self.reqwest.execute(request).await.with_context(|| {
                format!("querying streaming availability for {:?}", imdb_id)
            })?;
        let status = response.status();
        if !status.is_success() {
            let response_body =
                response.text().await.context("error reading response body")?;
            bail!(
                "unexpected error querying streaming availability for {:?}: \
                status {}, body {:?}",
                imdb_id,
                status,
                response_body,
            );
        }

        let result: Availability = response
            .json()
            .await
            .context("parsing streaming availability response body")?;
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
    streaming_info: Streaming,
}

#[derive(Clone, Debug, Deserialize)]
struct Streaming {
    // XXX working here: fill this in
    // See local-data
    // Example:
    // "hulu": {
    //  "us": {
    //    "link": "https://www.hulu.com/movie/805ca580-3372-4f7b-be6e-aecc78c2599f",
    //    "added": 1641121011,
    //    "leaving": 1648796340
    //  }
    //},
}
