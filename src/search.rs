use std::error::Error;

use isocountry::CountryCode;
use serde::de::DeserializeOwned;

use crate::{
    album::SimpleAlbum, artist::Artist, object::PagingObject, playlist::SimplePlaylist,
    track::Track, RequestClient,
};

#[derive(Clone, Debug, Default)]
pub struct SearchClient {
    client: RequestClient,
    query: Vec<String>,
    limit: Option<u32>,
    offset: Option<u32>,
    market: Option<CountryCode>,
}

impl SearchClient {
    pub fn new(access_token: &str, refresh_token: &str) -> Self {
        SearchClient {
            client: RequestClient::new(access_token, refresh_token),
            query: Vec::new(),
            ..Default::default()
        }
    }

    pub fn set_keyword(&mut self, keyword: &str) -> &mut Self {
        self.query.push(keyword.to_string());
        self
    }

    pub fn set_album_matching(&mut self, name: &str) -> &mut Self {
        self.set_matching(ObjectType::Album, name);
        self
    }

    pub fn set_artist_matching(&mut self, name: &str) -> &mut Self {
        self.set_matching(ObjectType::Artist, name);
        self
    }

    pub fn set_track_matching(&mut self, name: &str) -> &mut Self {
        self.set_matching(ObjectType::Track, name);
        self
    }

    pub fn set_matching(&mut self, object_type: ObjectType, name: &str) -> &mut Self {
        let query = format!("{}:{}", object_type.to_string(), name);
        self.query.push(query);
        self
    }

    pub fn set_year(&mut self, year: u64) -> &mut Self {
        let query = format!("year:{}", year);
        self.query.push(query);
        self
    }

    pub fn set_year_range(&mut self, range: (u64, u64)) -> &mut Self {
        let query = format!("{}-{}", range.0, range.1);
        self.query.push(query);
        self
    }

    pub fn set_limit(&mut self, limit: u32) -> &mut Self {
        self.limit = Some(limit);
        self
    }

    pub fn set_offset(&mut self, offset: u32) -> &mut Self {
        self.offset = Some(offset);
        self
    }

    pub fn set_market(&mut self, market: CountryCode) -> &mut Self {
        self.market = Some(market);
        self
    }

    pub async fn search_album(&mut self) -> Result<PagingObject<SimpleAlbum>, Box<dyn Error>> {
        self.search(ObjectType::Album).await
    }

    pub async fn search_artist(&mut self) -> Result<PagingObject<Artist>, Box<dyn Error>> {
        self.search(ObjectType::Artist).await
    }

    pub async fn search_playlist(
        &mut self,
    ) -> Result<PagingObject<SimplePlaylist>, Box<dyn Error>> {
        self.search(ObjectType::Playlist).await
    }

    pub async fn search_track(&mut self) -> Result<PagingObject<Track>, Box<dyn Error>> {
        self.search(ObjectType::Track).await
    }

    async fn search<T: DeserializeOwned + Clone>(
        &mut self,
        object_type: ObjectType,
    ) -> Result<T, Box<dyn Error>> {
        let builder = reqwest::Client::new()
            .get("https://api.spotify.com/v1/search")
            .query(&[("q", &self.to_query()), ("type", &object_type.to_string())]);

        let response = self
            .client
            .set_limit(self.limit)
            .set_offset(self.offset)
            .set_market(self.market)
            .send(builder)
            .await?
            .unwrap();

        let mut value: serde_json::Value = serde_json::from_str(&response.text().await?).unwrap();
        let key = format!("{}s", object_type.to_string());

        Ok(serde_json::from_value(value[key].take())?)
    }

    fn to_query(&self) -> String {
        self.query.join(" ")
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ObjectType {
    Album,
    Artist,
    Playlist,
    Track,
}

impl std::fmt::Display for ObjectType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ObjectType::Album => write!(f, "album"),
            ObjectType::Artist => write!(f, "artist"),
            ObjectType::Playlist => write!(f, "playlist"),
            ObjectType::Track => write!(f, "track"),
        }
    }
}
