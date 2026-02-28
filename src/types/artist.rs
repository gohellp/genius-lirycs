use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Artist {
    pub id: i32,
    pub name: String,
    pub header_image_url: String, //TODO: change type to `Url`
    pub image_url: String, //TODO: change type to `Url`
    pub url: String //TODO: change type to `Url`
}
