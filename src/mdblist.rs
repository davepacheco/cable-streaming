//! Client interfaces for MDBList API
//! See https://rapidapi.com/linaspurinis/api/mdblist/

use anyhow::bail;
use anyhow::Context;
use http::HeaderValue;
use serde::Deserialize;

const API_HOST: &str = "mdblist.p.rapidapi.com";

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
                .context("invalid mdblist API key")?,
        );

        let reqwest = reqwest::ClientBuilder::new()
            .default_headers(headers)
            .build()
            .context("initializing reqwest client")?;

        Ok(Client { reqwest })
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
        let request =
            self.reqwest.get(url).build().context("failed to build request")?;
        let response = self
            .reqwest
            .execute(request)
            .await
            .with_context(|| format!("querying mdblist for {:?}", title))?;
        let status = response.status();
        if !status.is_success() {
            let response_body =
                response.text().await.context("error reading response body")?;
            bail!(
                "unexpected error querying mdblist for {:?}: \
                status {}, body {:?}",
                title,
                status,
                response_body,
            );
        }

        let response_text =
            response.text().await.context("reading mdblist response body")?;
        let result: SearchResult = serde_json::from_str(&response_text)
            .with_context(|| {
                format!(
                    "parsing mdblist response body:\n----\n{}\n----\n",
                    response_text
                )
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
    results
        .into_iter()
        .filter_map(|m| {
            (m.year.is_some() && m.imdbid.is_some() && m.score > 0)
                .then(|| m.clone())
        })
        .collect()
}
