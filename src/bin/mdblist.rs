use std::path::PathBuf;

use anyhow::Context;
use cable_streaming::mdblist::prune;
use clap::Parser;

#[derive(Parser)]
/// low-level tool for querying MDBList API by movie title
struct Mdblist {
    #[clap(long, default_value = "creds.toml")]
    /// path to file containing credentials for these tools
    cred_file: PathBuf,
    /// Movie title to search for
    title: String,
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let mdblist = Mdblist::parse();

    let creds_path = &mdblist.cred_file;
    let creds_file = std::fs::read_to_string(creds_path)
        .with_context(|| format!("open {:?}", creds_path.display()))?;
    let creds: cable_streaming::Credentials = toml::from_str(&creds_file)
        .with_context(|| format!("parse {:?}", creds_path.display()))?;

    let client = cable_streaming::mdblist::Client::new(&creds.rapidapi_key)?;
    let results = client.title_lookup(&mdblist.title).await?;
    let results = prune(&results);

    println!("query: {:?}", &mdblist.title);
    println!("matches: {}", results.len());
    for m in results {
        println!("    score = {}, imdbid = {}, year = {}, title = {}",
            m.score, m.imdbid.unwrap(), m.year.unwrap(), m.title);
    }
    Ok(())
}
