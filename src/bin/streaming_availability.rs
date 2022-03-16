use clap::Parser;
use std::path::PathBuf;
use std::sync::Arc;

#[derive(Parser)]
/// low-level tool for querying Streaming Availbility API by IMDb id
struct Sa {
    #[clap(long, default_value = "cache.db")]
    /// path used for caching API responses
    cache: PathBuf,
    #[clap(long, default_value = "creds.toml")]
    /// path to file containing credentials for these tools
    cred_file: PathBuf,
    /// IMDB id of movie to search for (e.g., tt4846340)
    imdb_id: String,
}

// TODO commonize with mdblist

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let sa = Sa::parse();
    let creds =
        cable_streaming::credentials::Credentials::from_file(&sa.cred_file)?;
    let imdbid = &sa.imdb_id;
    let cache = cable_streaming::cache::RequestCache::new(&sa.cache)?;
    let client = cable_streaming::streaming_availability::Client::new(
        &creds.rapidapi_key,
        Arc::new(cache),
    )?;
    let result = client.lookup(imdbid).await?;

    println!("query: {:?}", imdbid);
    println!("result: {:?}", result);
    println!("services: {}", result.services().join(", "));
    Ok(())
}
