use std::{error::Error, fmt};

use futures::future::{BoxFuture, FutureExt};
use isocountry::CountryCode;
use serde::{Deserialize, Serialize};

use crate::{
    album::SimpleAlbum,
    object::{Follower, Image, PagingObject},
    track::Track,
    RequestClient,
};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Artist {
    pub followers: Follower,
    pub genres: Vec<String>,
    pub href: String,
    pub id: String,
    pub images: Vec<Image>,
    pub name: String,
    pub popularity: Option<u32>,
    #[serde(rename = "type")]
    pub object_type: String,
    pub uri: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct SimpleArtist {
    pub href: String,
    pub id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub object_type: String,
    pub uri: String,
}

#[derive(Debug)]
pub struct ArtistClient {
    client: RequestClient,
}

impl ArtistClient {
    pub fn new(access_token: &str, refresh_token: &str) -> Self {
        ArtistClient {
            client: RequestClient::new(access_token, refresh_token),
        }
    }

    pub async fn get_artist(
        &mut self,
        request: GetArtistRequest,
    ) -> Result<Artist, Box<dyn Error>> {
        let url = format!("https://api.spotify.com/v1/artists/{}", request.id);
        let builder = reqwest::Client::new().get(&url);
        let response = self.client.send(builder).await?.unwrap();

        Ok(response.json().await?)
    }

    pub fn get_artists(
        &mut self,
        mut request: GetArtistListRequest,
    ) -> BoxFuture<'_, Result<GetArtistListResponse, Box<dyn Error>>> {
        async move {
            let mut artist_response = GetArtistListResponse::default();

            if request.ids.len() > 50 {
                let drained: Vec<String> = request.ids.drain(..20).collect();
                let drained_request = GetArtistListRequest { ids: drained };
                artist_response
                    .artists
                    .append(&mut self.get_artists(drained_request).await?.artists);
                artist_response
                    .artists
                    .append(&mut self.get_artists(request.clone()).await?.artists);

                return Ok(artist_response);
            }

            let builder = reqwest::Client::new()
                .get("https://api.spotify.com/v1/artists")
                .query(&[("ids", request.ids.join(","))]);

            let response = self.client.send(builder).await?.unwrap();

            let mut values: GetArtistListResponse = response.json().await?;
            artist_response.artists.append(&mut values.artists);

            Ok(artist_response)
        }
        .boxed()
    }

    pub async fn get_albums(
        &mut self,
        request: GetArtistAlbumRequest,
    ) -> Result<PagingObject<SimpleAlbum>, Box<dyn Error>> {
        let url = format!("https://api.spotify.com/v1/artists/{}/albums", request.id);

        let query = if let Some(groups) = request.include_groups {
            let s = groups
                .into_iter()
                .map(|s| s.to_string())
                .collect::<Vec<String>>()
                .join(",");

            vec![("include_groups", s)]
        } else {
            Vec::new()
        };

        let builder = reqwest::Client::new().get(&url).query(&query);

        let response = self
            .client
            .set_offset(request.offset)
            .set_limit(request.limit)
            .set_country(request.country)
            .send(builder)
            .await?
            .unwrap();

        Ok(response.json().await?)
    }

    pub async fn get_top_tracks(
        &mut self,
        request: GetArtistTopTrackRequest,
    ) -> Result<GetArtistTopTrackResponse, Box<dyn Error>> {
        let url = format!(
            "https://api.spotify.com/v1/artists/{}/top-tracks",
            request.id,
        );

        let country = request
            .country
            .map_or("from_token".to_string(), |v| v.alpha2().to_string());

        let builder = reqwest::Client::new()
            .get(&url)
            .query(&[("country", country)]);

        let response = self.client.send(builder).await?.unwrap();

        Ok(response.json().await?)
    }

    pub async fn get_related_artists(
        &mut self,
        request: GetRelatedArtistRequest,
    ) -> Result<GetRelatedArtistResponse, Box<dyn Error>> {
        let url = format!(
            "https://api.spotify.com/v1/artists/{}/related-artists",
            request.id,
        );

        let builder = reqwest::Client::new().get(&url);
        let response = self.client.send(builder).await?.unwrap();

        Ok(response.json().await?)
    }
}

#[derive(Clone, Debug, Default, Deserialize)]
pub struct GetArtistRequest {
    pub id: String,
}

#[derive(Clone, Debug, Default, Deserialize)]
pub struct GetArtistListRequest {
    pub ids: Vec<String>,
}

#[derive(Clone, Debug, Default, Deserialize)]
pub struct GetArtistListResponse {
    pub artists: Vec<Artist>,
}

#[derive(Clone, Debug, Default)]
pub struct GetArtistAlbumRequest {
    pub id: String,
    pub include_groups: Option<Vec<IncludeGroup>>,
    pub country: Option<CountryCode>,
    pub limit: Option<u32>,
    pub offset: Option<u32>,
}

#[derive(Clone, Debug, Default)]
pub struct GetArtistTopTrackRequest {
    pub id: String,
    pub country: Option<CountryCode>,
}

#[derive(Clone, Debug, Default, Deserialize)]
pub struct GetArtistTopTrackResponse {
    pub tracks: Vec<Track>,
}

#[derive(Clone, Debug, Default)]
pub struct GetRelatedArtistRequest {
    pub id: String,
}

#[derive(Clone, Debug, Default, Deserialize)]
pub struct GetRelatedArtistResponse {
    pub artists: Vec<Artist>,
}

#[derive(Clone, Debug)]
pub enum IncludeGroup {
    Album,
    Single,
    AppearsOn,
    Compilation,
}

impl fmt::Display for IncludeGroup {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            IncludeGroup::Album => write!(f, "album"),
            IncludeGroup::Single => write!(f, "single"),
            IncludeGroup::AppearsOn => write!(f, "appears_on"),
            IncludeGroup::Compilation => write!(f, "compilation"),
        }
    }
}
