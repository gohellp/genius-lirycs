use serde_derive::{Deserialize, Serialize};

use super::Artist;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SongResponse {
    pub song: Song
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Song {
    pub id: i32,
    pub primary_artist: Artist,
    pub title: String,
    #[serde(rename = "language")]
    pub language_code: String,
    #[serde(rename = "song_art_image_thumbnail_url")]
    pub thumbnail_uri: String, //TODO: change type to `Url`
    pub url: String, //TODO: change type to `Url`
    pub album: Option<Album>,
    #[serde(rename = "producer_artists")]
    pub producers: Vec<Artist>,
    #[serde(rename = "writer_artists")]
    pub writers: Vec<Artist>,
    pub featured_artists: Vec<String>,
    pub media: Vec<Media>,
    #[serde(rename = "translation_songs")]
    pub translations: Vec<Translation>
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Album {
    pub id: i32,
    pub name: String,
    #[serde(rename = "cover_art_url")]
    pub cover_url: String, //TODO: change type to `Url`
    pub url: String, //TODO: change type to `Url`
    pub artist: Artist,
    #[serde(rename = "release_date_for_display")]
    pub release_date: String
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Media {
    pub provider: String,
    pub start: Option<i32>,
    pub native_uri: Option<String>, //TODO: change type to `Url`,
    pub url: String, //TODO: change type to `Url`,
    pub r#type: String
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Translation {
    pub id: i32,
    #[serde(rename = "language")]
    pub language_code: String,
    pub lyrics_state: String,
    pub title: String,
    pub url: String //TODO: change type to `Url`
}
