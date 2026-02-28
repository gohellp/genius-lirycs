mod search_results;
mod artist;
mod song;

pub use self::{
    search_results::*,
    artist::*,
    song::*,
};

#[derive(Debug, Clone, serde_derive::Deserialize, serde_derive::Serialize)]
pub struct Results<T> {
    pub meta: Meta,
    pub response: T
}
