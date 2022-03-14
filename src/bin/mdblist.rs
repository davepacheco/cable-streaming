use anyhow::Context;
use cable_streaming::mdblist::prune;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let creds_path = "creds.toml";
    let creds_file = std::fs::read_to_string(creds_path)
        .with_context(|| format!("open {:?}", creds_path))?;
    let creds: cable_streaming::Credentials = toml::from_str(&creds_file)
        .with_context(|| format!("parse {:?}", creds_path))?;

    // TODO command-line argument
    let title = "A Few Good Men";
    let client = cable_streaming::mdblist::Client::new(&creds.rapidapi_key)?;
    let results = client.title_lookup(title).await?;
    let results = prune(&results);

    println!("query: {:?}", title);
    println!("matches: {}", results.len());
    for m in results {
        println!("    score = {}, imdbid = {}, year = {}, title = {}",
            m.score, m.imdbid, m.year.unwrap(), m.title);
    }
    Ok(())
}
