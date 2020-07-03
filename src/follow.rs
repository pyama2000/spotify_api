use std::error::Error;

use futures::future::{BoxFuture, FutureExt};
use reqwest::header::CONTENT_TYPE;
use serde::Deserialize;
use serde_json::json;

use crate::{artist::Artist, object::CursorPagingObject, RequestClient};

#[derive(Clone, Debug, Default)]
pub struct FollowClient {
    client: RequestClient,
}

impl FollowClient {
    pub fn new(access_token: &str, refresh_token: &str) -> Self {
        FollowClient {
            client: RequestClient::new(access_token, refresh_token),
        }
    }

    pub async fn is_following_artist(
        &mut self,
        request: CheckFollowRequest,
    ) -> Result<Vec<bool>, Box<dyn Error>> {
        self.is_following(ObjectType::Artist, request.ids).await
    }

    pub async fn is_following_user(
        &mut self,
        request: CheckFollowRequest,
    ) -> Result<Vec<bool>, Box<dyn Error>> {
        self.is_following(ObjectType::User, request.ids).await
    }

    fn is_following(
        &mut self,
        object_type: ObjectType,
        mut ids: Vec<String>,
    ) -> BoxFuture<'_, Result<Vec<bool>, Box<dyn Error>>> {
        async move {
            let mut results = Vec::new();

            if ids.len() > 50 {
                let drained: Vec<String> = ids.drain(..50).collect();
                results.append(&mut self.is_following(object_type, drained).await?);
                results.append(&mut self.is_following(object_type, ids.clone()).await?);
            }

            let params = [("type", object_type.to_string()), ("ids", ids.join(","))];

            let builder = reqwest::Client::new()
                .get("https://api.spotify.com/v1/me/following/contains")
                .query(&params);

            let response = self.client.send(builder).await?.unwrap();

            results.append(&mut response.json().await?);

            Ok(results)
        }
        .boxed()
    }

    pub fn is_users_following_playlist(
        &mut self,
        mut request: CheckUserFollowPlaylistRequest,
    ) -> BoxFuture<'_, Result<Vec<bool>, Box<dyn Error>>> {
        async move {
            let url = format!(
                "https://api.spotify.com/v1/playlists/{}/followers/contains",
                request.playlist_id
            );

            let mut results = Vec::new();
            if request.user_ids.len() > 5 {
                let user_ids: Vec<String> = request.user_ids.drain(..5).collect();

                let r = CheckUserFollowPlaylistRequest {
                    playlist_id: request.playlist_id.clone(),
                    user_ids,
                };

                results.append(&mut self.is_users_following_playlist(r).await?);
                results.append(&mut self.is_users_following_playlist(request.clone()).await?);
            }

            let builder = reqwest::Client::new()
                .get(&url)
                .query(&[("ids", request.user_ids.join(","))]);

            let response = self.client.send(builder).await?.unwrap();

            let mut values: Vec<bool> = response.json().await?;
            results.append(&mut values);

            Ok(results)
        }
        .boxed()
    }

    pub async fn follow_artists(&mut self, request: FollowRequest) -> Result<(), Box<dyn Error>> {
        self.follow(ObjectType::Artist, request.ids).await
    }

    pub async fn follow_users(&mut self, request: FollowRequest) -> Result<(), Box<dyn Error>> {
        self.follow(ObjectType::User, request.ids).await
    }

    fn follow(
        &mut self,
        object_type: ObjectType,
        mut ids: Vec<String>,
    ) -> BoxFuture<'_, Result<(), Box<dyn Error>>> {
        async move {
            if ids.len() > 50 {
                self.follow(object_type, ids.drain(..50).collect())
                    .await?;
                self.follow(object_type, ids.clone()).await?;
            }


            let builder = reqwest::Client::new()
                .put("https://api.spotify.com/v1/me/following")
                .header(CONTENT_TYPE, "application/json")
                .query(&[("type", object_type.to_string())])
                .json(&json!({ "ids": ids }));

            self.client.send(builder).await?.unwrap();

            Ok(())
        }
        .boxed()
    }

    pub async fn follow_playlist(
        &mut self,
        request: FollowPlaylistRequest,
    ) -> Result<(), Box<dyn Error>> {
        let url = format!(
            "https://api.spotify.com/v1/playlists/{}/followers",
            request.id
        );

        let public = request.public.unwrap_or(true);
        let builder = reqwest::Client::new()
            .put(&url)
            .header(CONTENT_TYPE, "application/json")
            .json(&[("public", public)]);

        self.client.send(builder).await?.unwrap();

        Ok(())
    }

    pub async fn get_followed_artists(
        &mut self,
        request: GetUserFollowedArtistRequest,
    ) -> Result<GetUserFollowedArtistResponse, Box<dyn Error>> {
        let mut query = Vec::new();

        query.push(("type", request.object_type.to_string()));

        if let Some(after) = request.after {
            query.push(("after", after));
        }

        let builder = reqwest::Client::new()
            .get("https://api.spotify.com/v1/me/following")
            .query(&query);

        let response = self
            .client
            .set_limit(request.limit)
            .send(builder)
            .await?
            .unwrap();

        Ok(response.json().await?)
    }

    pub async fn unfollow_artists(
        &mut self,
        request: UnfollowRequest,
    ) -> Result<(), Box<dyn Error>> {
        self.unfollow(ObjectType::Artist, request.ids).await
    }

    pub async fn unfollow_users(&mut self, request: UnfollowRequest) -> Result<(), Box<dyn Error>> {
        self.unfollow(ObjectType::User, request.ids).await
    }

    fn unfollow(
        &mut self,
        object_type: ObjectType,
        mut ids: Vec<String>,
    ) -> BoxFuture<'_, Result<(), Box<dyn Error>>> {
        async move {
            if ids.len() > 50 {
                let drained: Vec<String> = ids.drain(..50).collect();
                self.unfollow(object_type.clone(), drained).await?;
                self.unfollow(object_type, ids.clone()).await?;
            }

            let builder = reqwest::Client::new()
                .delete("https://api.spotify.com/v1/me/following")
                .query(&[("type", object_type.to_string())])
                .json(&json!({ "ids": ids }));

            self.client.send(builder).await?.unwrap();

            Ok(())
        }
        .boxed()
    }

    pub async fn unfollow_playlist(
        &mut self,
        request: UnfollowPlaylistRequest,
    ) -> Result<(), Box<dyn Error>> {
        let url = format!(
            "https://api.spotify.com/v1/playlists/{}/followers",
            request.id,
        );

        let builder = reqwest::Client::new().delete(&url);

        self.client.send(builder).await?.unwrap();

        Ok(())
    }
}

#[derive(Clone, Debug, Default)]
pub struct CheckFollowRequest {
    pub ids: Vec<String>,
}

#[derive(Clone, Debug, Default)]
pub struct CheckUserFollowPlaylistRequest {
    pub playlist_id: String,
    pub user_ids: Vec<String>,
}

#[derive(Clone, Debug, Default)]
pub struct FollowRequest {
    pub ids: Vec<String>,
}

#[derive(Clone, Debug, Default)]
pub struct FollowPlaylistRequest {
    pub id: String,
    pub public: Option<bool>,
}

#[derive(Clone, Debug, Default)]
pub struct GetUserFollowedArtistRequest {
    pub object_type: ObjectType,
    pub limit: Option<u32>,
    pub after: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize)]
pub struct GetUserFollowedArtistResponse {
    pub artists: CursorPagingObject<Artist>,
}

#[derive(Clone, Debug, Default)]
pub struct UnfollowRequest {
    pub ids: Vec<String>,
}

#[derive(Clone, Debug, Default)]
pub struct UnfollowPlaylistRequest {
    pub id: String,
}

#[derive(Copy, Clone, Debug)]
pub enum ObjectType {
    Artist,
    User,
}

impl std::fmt::Display for ObjectType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ObjectType::Artist => write!(f, "artist"),
            ObjectType::User => write!(f, "user"),
        }
    }
}

impl Default for ObjectType {
    fn default() -> Self {
        ObjectType::Artist
    }
}
