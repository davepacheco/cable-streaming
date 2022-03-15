use cable_streaming::mdblist::prune;
use clap::Parser;
use std::path::PathBuf;

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
    let creds = cable_streaming::credentials::Credentials::from_file(
        &mdblist.cred_file,
    )?;
    let client = cable_streaming::mdblist::Client::new(&creds.rapidapi_key)?;
    let results = client.title_lookup(&mdblist.title).await?;
    let results = prune(&results);

    println!("query: {:?}", &mdblist.title);
    println!("matches: {}", results.len());
    for m in results {
        println!(
            "    score = {}, imdbid = {}, year = {}, title = {}",
            m.score,
            m.imdbid.unwrap(),
            m.year.unwrap(),
            m.title
        );
    }
    Ok(())
}
