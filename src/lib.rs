pub mod types;

use anyhow::Ok;
use jsonxf::pretty_print;
use regex::Regex;
use reqwest::{Client, Response};
use scraper::{Html, Selector};
use serde_json::from_str;
use types::{Hit, HitsResponse, Results, Song, SongResponse};

const BASE_URL: &str = "https://api.genius.com";

#[derive(Debug, Clone)]
pub struct GeniusApiClient {
    token: String,
    selector: Selector,
    client: Client
}

impl GeniusApiClient {
    pub fn new(token: &str) -> Self {
        Self {
            token: token.to_string(),
            selector: Selector::parse(
                r#"div[class^="Lyrics__Container"]"#
            ).unwrap(),
            client: Client::new()
        }
    }

    pub async fn search_song(
        &self,
        artist: &str,
        title: &str,
        auth_header: Option<bool>
    ) -> anyhow::Result<Vec<Hit>> {
        let auth = auth_header.unwrap_or(false);
        let mut query = format!("{BASE_URL}/search?q={artist} {title}");

        if !auth {
            query += &format!("&access_token={}",self.token);
        }
        let response: Response;

        if auth {
            response =
                self.client
                    .get(query)
                    .bearer_auth(&self.token)
                    .send()
                    .await?;
        } else {
            response = self.client.get(query).send().await?;
        }

        if response.status().is_success() {
            let temp = response.text().await?;
            let result: Results<HitsResponse> = from_str(&pretty_print(&temp).unwrap()).unwrap();
            Ok(result.response.hits)
        } else {
            Err(response.error_for_status().unwrap_err().into())
        }
    }

    pub async fn get_song(
        &self,
        artist: &str,
        title: &str,
        auth_header: Option<bool>
    ) -> anyhow::Result<Song> {
        let search_results =
            self.search_song(
                artist,
                title,
                auth_header
            ).await?;

        Ok(self.get_song_by_id(search_results[0].song.id).await?)
    }

    pub async fn get_song_by_id(&self, song_id: i32) -> anyhow::Result<Song> {
        let query = format!("{BASE_URL}/songs/{song_id}");
        let response =
            self.client
                .get(query)
                .bearer_auth(&self.token)
                .send()
                .await?;

        if response.status().is_success() {
            let response_text = response.text().await?;
            let result: Results<SongResponse> = from_str(&response_text)?;
            Ok(result.response.song)
        } else {
            Err(response.error_for_status().unwrap_err().into())
        }
    }

    pub async fn get_lyrics(&self, song_url: String) -> anyhow::Result<String> {
        let response = self.client.get(song_url).send().await?;

        if response.status().is_success() {
            let document = Html::parse_document(&response.text().await?);
            Ok(self.extract_lyrics(document)?)
        } else {
            todo!("Return an error on unsuccess code")
        }
    }

    fn extract_lyrics(&self, document: Html) -> anyhow::Result<String> {
        let selected_iter = document.select(&self.selector);

        let tags_selector = Regex::new(r#"<(?:"[^"]*"['"]*|'[^']*'['"]*|[^'">])+>"#).unwrap();
        let description_selector = Regex::new(r#"(?ms)(?:<div.*?</a></div></div>)"#).unwrap();

        let mut lyrics = String::new();

        for selected in selected_iter {
            let mut lyrics_without_brs = selected
                .inner_html()
                .replace("<br>", "\n");

            if lyrics_without_brs.contains("ContributorsCreditSong") {
                lyrics_without_brs = description_selector.replace_all(&lyrics_without_brs, "").to_string();
            }

            lyrics += &tags_selector.replace_all(&lyrics_without_brs, "").to_string();
            lyrics += "\n";
        }

        Ok(lyrics)

    }
}
