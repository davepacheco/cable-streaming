//! Too-simple cache for HTTP request responses
// This would be better as a proper caching HTTP server, where it could handle
// all the real considerations around request caching (which parameters imply
// different resources, expiration, etc.)

use anyhow::bail;
use anyhow::Context;
use serde::Deserialize;
use serde::Serialize;
use std::path::Path;

pub struct RequestCache {
    db: sled::Db,
}

impl RequestCache {
    pub fn new(path: &Path) -> Result<RequestCache, anyhow::Error> {
        Ok(RequestCache {
            db: sled::open(path)
                .with_context(|| format!("open sled db {:?}", path))?,
        })
    }

    pub fn key<'a, 'b>(&'a self, url: &'b reqwest::Url) -> String {
        url.as_str().to_string()
    }

    pub async fn request(
        &self,
        client: &reqwest::Client,
        url: reqwest::Url,
    ) -> Result<CachedResponse, anyhow::Error> {
        let key = self.key(&url);
        let cached_response = self
            .db
            .get(&key)
            .with_context(|| format!("looking up cache key {:?}", key))?;
        if let Some(serialized) = cached_response {
            eprintln!("cache hit");
            return Ok(serde_json::from_slice(&serialized)
                .context("deserializing cached response")?);
        }

        eprintln!("cache miss");
        let request =
            client.get(url).build().context("failed to build request")?;
        let response = client
            .execute(request)
            .await
            .with_context(|| format!("querying {:?}", key))?;
        let status = response.status();
        let response_body =
            response.text().await.context("error reading response body")?;
        if !status.is_success() {
            bail!(
                "unexpected error querying {:?}: status {}, body {:?}",
                key,
                status,
                response_body,
            );
        }

        let cached_response = CachedResponse {
            time_created: chrono::Utc::now(),
            body: response_body,
        };

        self.db
            .insert(
                &key,
                serde_json::to_vec(&cached_response)
                    .context("serializing cached response")?
                    .as_slice(),
            )
            .context("inserting cached response")?;

        Ok(cached_response)
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CachedResponse {
    pub time_created: chrono::DateTime<chrono::Utc>,
    // TODO headers
    pub body: String,
}
