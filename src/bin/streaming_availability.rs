use anyhow::Context;

// TODO commonize with mdblist

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let creds_path = "creds.toml";
    let creds_file = std::fs::read_to_string(creds_path)
        .with_context(|| format!("open {:?}", creds_path))?;
    let creds: cable_streaming::Credentials = toml::from_str(&creds_file)
        .with_context(|| format!("parse {:?}", creds_path))?;

    // TODO command-line argument
    // let imdbid = "tt0104257"; // "A Few Good Men"
    let imdbid = "tt1605783"; // "Midnight in Paris" (on Netflix)
    let client = cable_streaming::streaming_availability::Client::new(
        &creds.rapidapi_key,
    )?;
    let result = client.lookup(imdbid).await?;

    println!("query: {:?}", imdbid);
    println!("result: {:?}", result);
    Ok(())
}
