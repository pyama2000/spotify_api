use crate::object::{Artist, ObjectType, PagingObject, PagingObjectWrapper, Track};
use crate::{generate_params, Client};
use reqwest;
use serde::de::DeserializeOwned;
use std::default::Default;
use std::fmt;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum TimeRange {
    LongTerm,
    MediumTerm,
    ShortTerm,
}

impl fmt::Display for TimeRange {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TimeRange::LongTerm => write!(f, "long_term"),
            TimeRange::MediumTerm => write!(f, "medium_term"),
            TimeRange::ShortTerm => write!(f, "short_term"),
        }
    }
}

impl Default for TimeRange {
    fn default() -> Self {
        TimeRange::MediumTerm
    }
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct PersonalizationClient {
    access_token: String,
    refresh_token: String,
}

impl Client for PersonalizationClient {
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

impl PersonalizationClient {
    pub fn new(access_token: &str, refresh_token: &str) -> Self {
        PersonalizationClient {
            access_token: access_token.to_string(),
            refresh_token: refresh_token.to_string(),
        }
    }

    pub fn get_user_top_artists(
        &mut self,
        limit: Option<u32>,
        offset: Option<u32>,
        time_range: Option<TimeRange>,
    ) -> PagingObjectWrapper<Artist> {
        self.get(ObjectType::Artists, limit, offset, time_range)
    }

    pub fn get_user_top_tracks(
        &mut self,
        limit: Option<u32>,
        offset: Option<u32>,
        time_range: Option<TimeRange>,
    ) -> PagingObjectWrapper<Track> {
        self.get(ObjectType::Tracks, limit, offset, time_range)
    }

    pub fn get<T: DeserializeOwned + Clone>(
        &mut self,
        object_type: ObjectType,
        limit: Option<u32>,
        offset: Option<u32>,
        time_range: Option<TimeRange>,
    ) -> PagingObjectWrapper<T> {
        let url = format!(
            "https://api.spotify.com/v1/me/top/{}",
            object_type.to_string()
        );

        let mut params = generate_params(limit, offset);
        let time_range = time_range.unwrap_or_default();
        params.push(("time_range", time_range.to_string()));
        let request = reqwest::Client::new().get(&url).query(&params);
        let mut response = self.send(request).unwrap();
        let paging_object: PagingObject<T> = response.json().unwrap();

        PagingObjectWrapper::new(
            paging_object,
            &self.get_access_token(),
            &self.get_refresh_token(),
        )
    }
}
