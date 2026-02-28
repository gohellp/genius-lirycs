use serde_derive::{Deserialize, Serialize};

use super::Artist;


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Meta {
    pub status: i32
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HitsResponse {
    pub hits: Vec<Hit>
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Hit {
    pub index: String,
    pub r#type: String,
    #[serde(rename = "result")]
    pub song: ResultSong
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResultSong {
    pub artist_names: String,
    pub id: i32,
    #[serde(rename = "release_date_for_display")]
    pub release_date: String,
    #[serde(rename = "song_art_image_url")]
    pub cover: String, //TODO: change type to `Url`
    pub title: String,
    pub url: String, //TODO: change type to `Url`
    #[serde(rename = "primary_artist")]
    pub artist: Artist,
    #[serde(rename = "primary_artists")]
    pub artists: Vec<Artist>,
    pub featured_artists: Vec<Artist>
}
