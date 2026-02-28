// use std::env::VarError;

use genius_lyrics::GeniusApiClient;

pub fn setup() -> GeniusApiClient {
    dotenvy::dotenv().ok();
    let token = std::env::var("GENIUS_TOKEN").expect("GENIUS_TOKEN should be defined");
    GeniusApiClient::new(&token)
}
