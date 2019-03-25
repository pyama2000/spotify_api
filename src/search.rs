use crate::object::{PagingObject, PagingObjectWrapper};
use crate::{generate_params, get_value, Client, CountryCode};
use serde::de::DeserializeOwned;
use std::convert::From;
use std::fmt;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum ObjectType {
    Album,
    Artist,
    Playlist,
    Track,
}

impl fmt::Display for ObjectType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ObjectType::Album => write!(f, "album"),
            ObjectType::Artist => write!(f, "artist"),
            ObjectType::Playlist => write!(f, "playlist"),
            ObjectType::Track => write!(f, "track"),
        }
    }
}

impl From<&str> for ObjectType {
    fn from(value: &str) -> Self {
        match value {
            "album" => ObjectType::Album,
            "artist" => ObjectType::Album,
            "playlist" => ObjectType::Playlist,
            "track" => ObjectType::Track,
            _ => ObjectType::Album,
        }
    }
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct SearchQuery {
    pub query: Vec<String>,
    pub object_type: ObjectType,
}

impl SearchQuery {
    pub fn new(object_type: &str) -> Self {
        SearchQuery {
            query: Vec::new(),
            object_type: ObjectType::from(object_type),
        }
    }

    pub fn set_keyword(&mut self, keyword: &str) {
        self.query.push(keyword.to_string());
    }

    pub fn set_album_matching(&mut self, name: &str) {
        self.set_matching(ObjectType::Album, name);
    }

    pub fn set_artist_matching(&mut self, name: &str) {
        self.set_matching(ObjectType::Artist, name);
    }

    pub fn set_track_matching(&mut self, name: &str) {
        self.set_matching(ObjectType::Track, name);
    }

    pub fn set_matching(&mut self, object_type: ObjectType, name: &str) {
        let query = format!("{}:{}", object_type.to_string(), name);
        self.query.push(query);
    }

    pub fn set_year(&mut self, year: u64) {
        let query = format!("year:{}", year);
        self.query.push(query);
    }

    pub fn set_year_range(&mut self, range: (u64, u64)) {
        let query = format!("{}-{}", range.0, range.1);
        self.query.push(query);
    }

    pub fn to_query(&self) -> String {
        self.query.join(" ")
    }
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct SearchClient {
    access_token: String,
    refresh_token: String,
}

impl Client for SearchClient {
    fn get_access_token(&self) -> String {
        self.access_token.to_string()
    }

    fn get_refresh_token(&self) -> String {
        self.refresh_token.to_string()
    }

    fn set_access_token(&mut self, access_token: &str) -> &mut Client {
        self.access_token = access_token.to_string();
        self
    }
}

impl SearchClient {
    pub fn new(access_token: &str, refresh_token: &str) -> Self {
        SearchClient {
            access_token: access_token.to_string(),
            refresh_token: refresh_token.to_string(),
        }
    }

    pub fn search<T: DeserializeOwned + Clone>(
        &mut self,
        query: SearchQuery,
        market: Option<CountryCode>,
        limit: Option<u32>,
        offset: Option<u32>,
    ) -> PagingObjectWrapper<T> {
        let mut params = generate_params(limit, offset);
        let q = &query.to_query();
        params.push(("q", q.to_string()));
        let object_type = &query.object_type;
        params.push(("type", object_type.to_string()));
        let market = market.map_or("from_token".to_string(), |v| v.alpha2().to_string());
        params.push(("market", market));
        let request = reqwest::Client::new()
            .get("https://api.spotify.com/v1/search")
            .query(&params);
        let mut response = self.send(request).unwrap();
        let json = response.text().unwrap();
        let key = format!("{}s", object_type.to_string());
        let paging_object: PagingObject<T> = get_value(&json, &key).unwrap();

        PagingObjectWrapper::new(
            paging_object,
            &self.get_access_token(),
            &self.get_refresh_token(),
        )
    }
}
