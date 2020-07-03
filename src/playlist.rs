use std::error::Error;

use chrono::{DateTime, Utc};
use futures::future::{BoxFuture, FutureExt};
use isocountry::CountryCode;
use reqwest::header::{HeaderMap, CONTENT_TYPE};
use serde::Deserialize;
use serde_json::json;

use crate::{
    object::{Follower, Image, PagingObject},
    track::Track,
    user::User,
    RequestClient,
};

#[derive(Clone, Debug, Deserialize)]
pub struct Playlist {
    pub collaborative: bool,
    pub description: Option<String>,
    pub followers: Follower,
    pub href: String,
    pub id: String,
    pub images: Vec<Image>,
    pub name: String,
    pub owner: User,
    pub public: Option<bool>,
    pub snapshot_id: String,
    pub tracks: PagingObject<PlaylistTrack>,
    #[serde(rename = "type")]
    pub object_type: String,
    pub uri: String,
}

#[derive(Clone, Debug, Default, Deserialize)]
pub struct SimplePlaylist {
    pub collaborative: bool,
    pub description: Option<String>,
    pub href: String,
    pub id: String,
    pub images: Vec<Image>,
    pub name: String,
    pub owner: User,
    pub public: Option<bool>,
    pub snapshot_id: String,
    #[serde(rename = "type")]
    pub object_type: String,
    pub uri: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct PlaylistTrack {
    pub added_at: DateTime<Utc>,
    pub added_by: User,
    pub is_local: bool,
    pub track: Track,
}

#[derive(Clone, Debug, Default)]
pub struct PlaylistClient {
    client: RequestClient,
}

impl PlaylistClient {
    pub fn new(access_token: &str, refresh_token: &str) -> Self {
        PlaylistClient {
            client: RequestClient::new(access_token, refresh_token),
        }
    }

    pub fn add_items(
        &mut self,
        mut request: AddItemsRequest,
    ) -> BoxFuture<'_, Result<Vec<Snapshot>, Box<dyn Error>>> {
        async move {
            let url = format!(
                "https://api.spotify.com/v1/playlists/{}/tracks",
                request.playlist_id
            );

            let mut results = Vec::new();
            if request.uris.len() > 100 {
                let uris: Vec<String> = request.uris.drain(..100).collect();

                let next_request = AddItemsRequest {
                    playlist_id: request.playlist_id.clone(),
                    uris,
                    position: request.position,
                };

                results.append(&mut self.add_items(next_request).await?);
                results.append(&mut self.add_items(request.clone()).await?);

                return Ok(results);
            }

            let mut json = serde_json::Map::new();
            json.insert("uris".to_string(), json!(request.uris));

            if let Some(position) = request.position {
                json.insert("position".to_string(), json!(position));
            }

            let mut headers = HeaderMap::new();
            headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());

            let builder = reqwest::Client::new()
                .post(&url)
                .headers(headers)
                .json(&json);

            let response = self.client.send(builder).await?.unwrap();
            let mut result = response.json().await?;
            results.append(&mut result);

            Ok(results)
        }
        .boxed()
    }

    pub async fn change_name(&mut self, request: ChangeNameRequest) -> Result<(), Box<dyn Error>> {
        let request = ChangeDetailRequest {
            playlist_id: request.playlist_id,
            name: Some(request.name),
            ..Default::default()
        };

        self.change_detail(request).await
    }

    pub async fn change_public(
        &mut self,
        request: ChangePublicRequest,
    ) -> Result<(), Box<dyn Error>> {
        let request = ChangeDetailRequest {
            playlist_id: request.playlist_id,
            public: Some(request.public),
            ..Default::default()
        };

        self.change_detail(request).await
    }

    pub async fn change_collaborative(
        &mut self,
        request: ChangeCollaborativeRequest,
    ) -> Result<(), Box<dyn Error>> {
        let request = ChangeDetailRequest {
            playlist_id: request.playlist_id,
            collaborative: Some(request.collaborative),
            ..Default::default()
        };

        self.change_detail(request).await
    }

    pub async fn change_description(
        &mut self,
        request: ChangeDescriptionRequest,
    ) -> Result<(), Box<dyn Error>> {
        let request = ChangeDetailRequest {
            playlist_id: request.playlist_id,
            description: Some(request.description),
            ..Default::default()
        };

        self.change_detail(request).await
    }

    pub async fn change_detail(
        &mut self,
        request: ChangeDetailRequest,
    ) -> Result<(), Box<dyn Error>> {
        let url = format!(
            "https://api.spotify.com/v1/playlists/{}",
            request.playlist_id
        );

        let mut json = serde_json::Map::new();
        if let Some(name) = request.name {
            json.insert("name".to_string(), json!(name));
        }

        if let Some(public) = request.public {
            json.insert("public".to_string(), json!(public));
        }

        if let Some(collaborative) = request.collaborative {
            json.insert("collaborative".to_string(), json!(collaborative));
        }

        if let Some(description) = request.description {
            json.insert("description".to_string(), json!(description));
        }

        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());

        let builder = reqwest::Client::new()
            .put(&url)
            .headers(headers)
            .json(&json);

        self.client.send(builder).await?.unwrap();

        Ok(())
    }

    pub async fn create_playlist(
        &mut self,
        request: CreatePlaylistRequest,
    ) -> Result<Playlist, Box<dyn Error>> {
        let url = format!(
            "https://api.spotify.com/v1/users/{}/playlists",
            request.user_id
        );

        let mut json = serde_json::Map::new();
        json.insert("name".to_string(), json!(request.name));

        if let Some(public) = request.public {
            json.insert("public".to_string(), json!(public));
        }

        if let Some(collaborative) = request.collaborative {
            json.insert("collaborative".to_string(), json!(collaborative));
        }

        if let Some(description) = request.description {
            json.insert("description".to_string(), json!(description));
        }

        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());

        let builder = reqwest::Client::new()
            .post(&url)
            .headers(headers)
            .json(&json);

        let response = self.client.send(builder).await?.unwrap();

        Ok(response.json().await?)
    }

    pub async fn get_playlists(
        &mut self,
        request: GetPlaylistsRequest,
    ) -> Result<PagingObject<SimplePlaylist>, Box<dyn Error>> {
        let url = if let Some(user_id) = request.user_id {
            format!("https://api.spotify.com/v1/users/{}/playlists", user_id)
        } else {
            "https://api.spotify.com/v1/me/playlists".to_string()
        };

        let builder = reqwest::Client::new().get(&url);

        let response = self
            .client
            .set_limit(request.limit)
            .set_offset(request.offset)
            .send(builder)
            .await?
            .unwrap();

        Ok(response.json().await?)
    }

    pub async fn get_image(
        &mut self,
        request: GetImageRequest,
    ) -> Result<Vec<Image>, Box<dyn Error>> {
        let url = format!(
            "https://api.spotify.com/v1/playlists/{}/images",
            request.playlist_id
        );

        let builder = reqwest::Client::new().get(&url);

        let response = self.client.send(builder).await?.unwrap();

        Ok(response.json().await?)
    }

    pub async fn get_playlist(
        &mut self,
        request: GetPlaylistRequest,
    ) -> Result<Playlist, Box<dyn Error>> {
        let url = format!(
            "https://api.spotify.com/v1/playlists/{}",
            request.playlist_id
        );

        let builder = reqwest::Client::new().get(&url);

        let response = self
            .client
            .set_market(request.market)
            .send(builder)
            .await?
            .unwrap();

        Ok(response.json().await?)
    }

    pub async fn get_tracks(
        &mut self,
        request: GetPlaylistTracksRequest,
    ) -> Result<PagingObject<PlaylistTrack>, Box<dyn Error>> {
        let url = format!(
            "https://api.spotify.com/v1/playlists/{}/tracks",
            request.playlist_id
        );

        let builder = reqwest::Client::new().get(&url);

        let response = self
            .client
            .set_limit(request.limit)
            .set_offset(request.offset)
            .set_market(request.market)
            .send(builder)
            .await?
            .unwrap();

        Ok(response.json().await?)
    }

    pub fn remove_items(
        &mut self,
        mut request: RemoveItemsRequest,
    ) -> BoxFuture<'_, Result<Vec<Snapshot>, Box<dyn Error>>> {
        async move {
            let url = format!(
                "https://api.spotify.com/v1/playlists/{}/tracks",
                request.playlist_id
            );

            let mut results = Vec::new();
            if request.tracks.len() > 100 {
                let tracks = request.tracks.drain(..100).collect();

                let next_request = RemoveItemsRequest {
                    playlist_id: request.playlist_id.clone(),
                    tracks,
                    snapshot_id: request.snapshot_id.clone(),
                };

                results.append(&mut self.remove_items(next_request).await?);
                results.append(&mut self.remove_items(request.clone()).await?);

                return Ok(results);
            }

            let mut json = serde_json::Map::new();
            let tracks = request
                .tracks
                .into_iter()
                .map(|(uri, positions)| {
                    let mut object = serde_json::Map::new();
                    object.insert("uri".to_string(), json!(uri));

                    if let Some(positions) = positions {
                        object.insert("positions".to_string(), json!(positions));
                    }

                    object
                })
                .collect();

            json.insert("tracks".to_string(), tracks);

            if let Some(snapshot_id) = request.snapshot_id {
                json.insert("snapshot_id".to_string(), json!(snapshot_id));
            }

            let mut headers = HeaderMap::new();
            headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());

            let builder = reqwest::Client::new()
                .delete(&url)
                .headers(headers)
                .json(&json);

            let response = self.client.send(builder).await?.unwrap();
            let mut result = response.json().await?;
            results.append(&mut result);

            Ok(results)
        }
        .boxed()
    }

    pub async fn reorder(&mut self, request: ReorderRequest) -> Result<Snapshot, Box<dyn Error>> {
        let url = format!(
            "https://api.spotify.com/v1/playlists/{}/tracks",
            request.playlist_id
        );

        let mut json = serde_json::Map::new();
        json.insert("range_start".to_string(), json!(request.range_start));
        json.insert("insert_before".to_string(), json!(request.insert_before));

        if let Some(length) = request.range_length {
            json.insert("range_length".to_string(), json!(length));
        }

        if let Some(snapshot_id) = request.snapshot_id {
            json.insert("snapshot_id".to_string(), json!(snapshot_id));
        }

        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());

        let builder = reqwest::Client::new()
            .put(&url)
            .headers(headers)
            .json(&json);

        let response = self.client.send(builder).await?.unwrap();

        Ok(response.json().await?)
    }

    pub async fn replace(&mut self, request: ReplaceRequest) -> Result<(), Box<dyn Error>> {
        let url = format!(
            "https://api.spotify.com/v1/playlists/{}/tracks",
            request.playlist_id
        );

        let json = json!({
            "uris": request.uris,
        });

        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());

        let builder = reqwest::Client::new()
            .put(&url)
            .headers(headers)
            .json(&json);

        self.client.send(builder).await?.unwrap();

        Ok(())
    }
}

#[derive(Clone, Debug, Default, Deserialize)]
pub struct Snapshot {
    snapshot_id: String,
}

#[derive(Clone, Debug, Default)]
pub struct AddItemsRequest {
    pub playlist_id: String,
    pub uris: Vec<String>,
    pub position: Option<u32>,
}

#[derive(Clone, Debug, Default)]
pub struct ChangeNameRequest {
    pub playlist_id: String,
    pub name: String,
}

#[derive(Clone, Debug, Default)]
pub struct ChangePublicRequest {
    pub playlist_id: String,
    pub public: bool,
}

#[derive(Clone, Debug, Default)]
pub struct ChangeCollaborativeRequest {
    pub playlist_id: String,
    pub collaborative: bool,
}

#[derive(Clone, Debug, Default)]
pub struct ChangeDescriptionRequest {
    pub playlist_id: String,
    pub description: String,
}

#[derive(Clone, Debug, Default)]
pub struct ChangeDetailRequest {
    pub playlist_id: String,
    pub name: Option<String>,
    pub public: Option<bool>,
    pub collaborative: Option<bool>,
    pub description: Option<String>,
}

#[derive(Clone, Debug, Default)]
pub struct CreatePlaylistRequest {
    pub user_id: String,
    pub name: String,
    pub public: Option<bool>,
    pub collaborative: Option<bool>,
    pub description: Option<String>,
}

#[derive(Clone, Debug, Default)]
pub struct GetPlaylistsRequest {
    pub user_id: Option<String>,
    pub limit: Option<u32>,
    pub offset: Option<u32>,
}

#[derive(Clone, Debug, Default)]
pub struct GetImageRequest {
    pub playlist_id: String,
}

#[derive(Clone, Debug, Default)]
pub struct GetPlaylistRequest {
    pub playlist_id: String,
    pub market: Option<CountryCode>,
}

#[derive(Clone, Debug, Default)]
pub struct GetPlaylistTracksRequest {
    pub playlist_id: String,
    pub limit: Option<u32>,
    pub offset: Option<u32>,
    pub market: Option<CountryCode>,
}

#[derive(Clone, Debug, Default)]
pub struct RemoveItemsRequest {
    pub playlist_id: String,
    pub tracks: Vec<(String, Option<Vec<u32>>)>,
    pub snapshot_id: Option<String>,
}

#[derive(Clone, Debug, Default)]
pub struct ReorderRequest {
    pub playlist_id: String,
    pub range_start: u32,
    pub range_length: Option<u32>,
    pub insert_before: u32,
    pub snapshot_id: Option<String>,
}

#[derive(Clone, Debug, Default)]
pub struct ReplaceRequest {
    pub playlist_id: String,
    pub uris: Vec<String>,
}
