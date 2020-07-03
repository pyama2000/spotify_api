use std::error::Error;

use serde::de::DeserializeOwned;

use crate::{artist::Artist, object::PagingObject, track::Track, RequestClient};

#[derive(Clone, Debug, Default)]
pub struct PersonalizationClient {
    client: RequestClient,
}

impl PersonalizationClient {
    pub fn new(access_token: &str, refresh_token: &str) -> Self {
        PersonalizationClient {
            client: RequestClient::new(access_token, refresh_token),
        }
    }

    pub async fn get_top_artists(
        &mut self,
        request: GetTopRequest,
    ) -> Result<PagingObject<Artist>, Box<dyn Error>> {
        self.get_top(ObjectType::Artists, request).await
    }

    pub async fn get_top_tracks(
        &mut self,
        request: GetTopRequest,
    ) -> Result<PagingObject<Track>, Box<dyn Error>> {
        self.get_top(ObjectType::Tracks, request).await
    }

    async fn get_top<T: DeserializeOwned + Clone>(
        &mut self,
        object_type: ObjectType,
        request: GetTopRequest,
    ) -> Result<PagingObject<T>, Box<dyn Error>> {
        let url = format!(
            "https://api.spotify.com/v1/me/top/{}",
            object_type.to_string()
        );

        let query = if let Some(time_range) = request.time_range {
            vec![("time_range", time_range.to_string())]
        } else {
            Vec::new()
        };

        let builder = reqwest::Client::new().get(&url).query(&query);
        let response = self
            .client
            .set_offset(request.offset)
            .set_limit(request.limit)
            .send(builder)
            .await?
            .unwrap();

        Ok(response.json().await?)
    }
}

#[derive(Clone, Debug, Default)]
pub struct GetTopRequest {
    pub offset: Option<u32>,
    pub limit: Option<u32>,
    pub time_range: Option<TimeRange>,
}

#[derive(Copy, Clone, Debug)]
pub enum TimeRange {
    LongTerm,
    MediumTerm,
    ShortTerm,
}

impl std::fmt::Display for TimeRange {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
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

#[derive(Copy, Clone, Debug)]
pub enum ObjectType {
    Artists,
    Tracks,
}

impl std::fmt::Display for ObjectType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ObjectType::Artists => write!(f, "artists"),
            ObjectType::Tracks => write!(f, "tracks"),
        }
    }
}
