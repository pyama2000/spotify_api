use std::error::Error;

use chrono::{DateTime, Utc};
use futures::future::{BoxFuture, FutureExt};
use isocountry::CountryCode;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::json;

use crate::{album::Album, object::PagingObject, track::Track, RequestClient};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct SavedAlbum {
    added_at: Option<DateTime<Utc>>,
    album: Album,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct SavedShow {
    added_at: Option<DateTime<Utc>>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct SavedTrack {
    added_at: Option<DateTime<Utc>>,
    track: Track,
}

#[derive(Clone, Debug, Default)]
pub struct LibraryClient {
    client: RequestClient,
}

impl LibraryClient {
    pub fn new(access_token: &str, refresh_token: &str) -> Self {
        LibraryClient {
            client: RequestClient::new(access_token, refresh_token),
        }
    }

    pub async fn is_saved_albums(
        &mut self,
        request: CheckSavedRequest,
    ) -> Result<Vec<bool>, Box<dyn Error>> {
        self.is_saved(ObjectType::Albums, request.ids).await
    }

    pub async fn is_saved_shows(
        &mut self,
        request: CheckSavedRequest,
    ) -> Result<Vec<bool>, Box<dyn Error>> {
        self.is_saved(ObjectType::Shows, request.ids).await
    }

    pub async fn is_saved_tracks(
        &mut self,
        request: CheckSavedRequest,
    ) -> Result<Vec<bool>, Box<dyn Error>> {
        self.is_saved(ObjectType::Tracks, request.ids).await
    }

    fn is_saved(
        &mut self,
        object_type: ObjectType,
        mut ids: Vec<String>,
    ) -> BoxFuture<'_, Result<Vec<bool>, Box<dyn Error>>> {
        async move {
            let url = format!(
                "https://api.spotify.com/v1/me/{}/contains",
                object_type.to_string()
            );

            let mut results = Vec::new();
            if ids.len() > 50 {
                results.append(
                    &mut self
                        .is_saved(object_type, ids.drain(..50).collect())
                        .await?,
                );

                results.append(&mut self.is_saved(object_type, ids.clone()).await?);

                return Ok(results);
            }

            let builder = reqwest::Client::new()
                .get(&url)
                .query(&[("ids", ids.join(","))]);

            let response = self.client.send(builder).await?.unwrap();
            results.append(&mut response.json().await?);

            Ok(results)
        }
        .boxed()
    }

    pub async fn get_saved_albums(
        &mut self,
        request: GetSavedRequest,
    ) -> Result<PagingObject<SavedAlbum>, Box<dyn Error>> {
        self.get_saved(ObjectType::Albums, request).await
    }

    pub async fn get_saved_shows(
        &mut self,
        request: GetSavedRequest,
    ) -> Result<PagingObject<SavedShow>, Box<dyn Error>> {
        self.get_saved(ObjectType::Shows, request).await
    }

    pub async fn get_saved_tracks(
        &mut self,
        request: GetSavedRequest,
    ) -> Result<PagingObject<SavedTrack>, Box<dyn Error>> {
        self.get_saved(ObjectType::Tracks, request).await
    }

    async fn get_saved<T: DeserializeOwned + Clone>(
        &mut self,
        object_type: ObjectType,
        request: GetSavedRequest,
    ) -> Result<PagingObject<T>, Box<dyn Error>> {
        let url = format!("https://api.spotify.com/v1/me/{}", object_type.to_string());

        let builder = reqwest::Client::new().get(&url);
        let response = self
            .client
            .set_offset(request.offset)
            .set_limit(request.limit)
            .set_market(request.market)
            .send(builder)
            .await?
            .unwrap();

        Ok(response.json().await?)
    }

    pub async fn remove_saved_albums(
        &mut self,
        request: RemoveSavedRequest,
    ) -> Result<(), Box<dyn Error>> {
        self.remove_saved(ObjectType::Albums, request.ids).await
    }

    pub async fn remove_saved_shows(
        &mut self,
        request: RemoveSavedRequest,
    ) -> Result<(), Box<dyn Error>> {
        self.remove_saved(ObjectType::Shows, request.ids).await
    }

    pub async fn remove_saved_tracks(
        &mut self,
        request: RemoveSavedRequest,
    ) -> Result<(), Box<dyn Error>> {
        self.remove_saved(ObjectType::Tracks, request.ids).await
    }

    fn remove_saved(
        &mut self,
        object_type: ObjectType,
        mut ids: Vec<String>,
    ) -> BoxFuture<'_, Result<(), Box<dyn Error>>> {
        async move {
            let url = format!("https://api.spotify.com/v1/me/{}", object_type.to_string());

            if ids.len() > 50 {
                self.remove_saved(object_type, ids.drain(..50).collect())
                    .await?;
                self.remove_saved(object_type, ids.clone()).await?;

                return Ok(());
            }

            let builder = reqwest::Client::new()
                .delete(&url)
                .json(&json!({ "ids": ids }));

            self.client.send(builder).await?.unwrap();

            Ok(())
        }
        .boxed()
    }

    pub async fn save_albums(&mut self, request: SaveRequest) -> Result<(), Box<dyn Error>> {
        self.save(ObjectType::Albums, request.ids).await
    }

    pub async fn save_shows(&mut self, request: SaveRequest) -> Result<(), Box<dyn Error>> {
        self.save(ObjectType::Shows, request.ids).await
    }

    pub async fn save_tracks(&mut self, request: SaveRequest) -> Result<(), Box<dyn Error>> {
        self.save(ObjectType::Tracks, request.ids).await
    }

    fn save(
        &mut self,
        object_type: ObjectType,
        mut ids: Vec<String>,
    ) -> BoxFuture<'_, Result<(), Box<dyn Error>>> {
        async move {
            let url = format!("https://api.spotify.com/v1/me/{}", object_type.to_string());

            if ids.len() > 50 {
                self.remove_saved(object_type, ids.drain(..50).collect())
                    .await?;
                self.remove_saved(object_type, ids.clone()).await?;
            }

            let builder = reqwest::Client::new()
                .put(&url)
                .json(&json!({ "ids": ids }));

            self.client.send(builder).await?.unwrap();

            Ok(())
        }
        .boxed()
    }
}

#[derive(Clone, Debug, Default)]
pub struct CheckSavedRequest {
    pub ids: Vec<String>,
}

#[derive(Clone, Debug, Default)]
pub struct GetSavedRequest {
    pub offset: Option<u32>,
    pub limit: Option<u32>,
    pub market: Option<CountryCode>,
}

#[derive(Clone, Debug, Default)]
pub struct RemoveSavedRequest {
    pub ids: Vec<String>,
}

#[derive(Clone, Debug, Default)]
pub struct SaveRequest {
    pub ids: Vec<String>,
}

#[derive(Copy, Clone, Debug)]
pub enum ObjectType {
    Albums,
    Shows,
    Tracks,
}

impl std::fmt::Display for ObjectType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ObjectType::Albums => write!(f, "albums"),
            ObjectType::Shows => write!(f, "shows"),
            ObjectType::Tracks => write!(f, "tracks"),
        }
    }
}
