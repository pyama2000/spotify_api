use std::error::Error;

use futures::future::{BoxFuture, FutureExt};
use isocountry::CountryCode;
use serde::Deserialize;

use crate::{artist::SimpleArtist, object::{Image, PagingObject}, track::SimpleTrack, RequestClient};

#[derive(Clone, Debug, Default, Deserialize)]
pub struct Album {
    pub album_type: String,
    pub artists: Vec<SimpleArtist>,
    pub available_markets: Vec<String>,
    pub genres: Vec<String>,
    pub href: String,
    pub id: String,
    pub images: Vec<Image>,
    pub label: String,
    pub name: String,
    pub popularity: u32,
    pub release_date: String,
    pub release_date_precision: String,
    pub tracks: PagingObject<SimpleTrack>,
    #[serde(rename = "type")]
    pub object_type: String,
    pub uri: String,
}

#[derive(Clone, Debug, Default, Deserialize)]
pub struct SimpleAlbum {
    pub album_group: Option<String>,
    pub album_type: String,
    pub artists: Vec<SimpleArtist>,
    pub available_markets: Option<Vec<String>>,
    pub href: String,
    pub id: String,
    pub images: Vec<Image>,
    pub name: String,
    #[serde(rename = "type")]
    pub object_type: String,
    pub uri: String,
}

#[derive(Debug)]
pub struct AlbumClient {
    client: RequestClient,
}

impl AlbumClient {
    pub fn new(access_token: &str, refresh_token: &str) -> AlbumClient {
        AlbumClient {
            client: RequestClient::new(access_token, refresh_token),
        }
    }

    pub async fn get_album(&mut self, request: GetAlbumRequest) -> Result<Album, Box<dyn Error>> {
        let url = format!("https://api.spotify.com/v1/albums/{}", request.id);
        let builder = reqwest::Client::new().get(&url);
        let response = self
            .client
            .set_market(request.market)
            .send(builder)
            .await?
            .unwrap();

        Ok(response.json().await?)
    }

    pub fn get_albums(
        &mut self,
        mut request: GetAlbumListRequest,
    ) -> BoxFuture<'_, Result<GetAlbumListResponse, Box<dyn Error>>> {
        async move {
            let mut albums_response = GetAlbumListResponse::default();

            if request.ids.len() > 20 {
                let drained: Vec<String> = request.ids.drain(..20).collect();
                let drained_request = GetAlbumListRequest {
                    ids: drained,
                    market: request.market,
                };
                albums_response
                    .albums
                    .append(&mut self.get_albums(drained_request).await?.albums);
                albums_response
                    .albums
                    .append(&mut self.get_albums(request.clone()).await?.albums);

                return Ok(albums_response);
            }

            let builder = reqwest::Client::new()
                .get("https://api.spotify.com/v1/albums")
                .query(&[("ids", request.ids.join(","))]);

            let response = self
                .client
                .set_market(request.market)
                .send(builder)
                .await?
                .unwrap();

            let mut values: GetAlbumListResponse = response.json().await?;
            albums_response.albums.append(&mut values.albums);

            Ok(albums_response)
        }
        .boxed()
    }

    pub async fn get_tracks(
        &mut self,
        request: GetTrackListRequest,
    ) -> Result<PagingObject<SimpleTrack>, Box<dyn Error>> {
        let url = format!("https://api.spotify.com/v1/albums/{}/tracks", request.id);
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
}

#[derive(Clone, Debug, Default, Deserialize)]
pub struct GetAlbumRequest {
    pub id: String,
    pub market: Option<CountryCode>,
}

#[derive(Clone, Debug, Default, Deserialize)]
pub struct GetAlbumListRequest {
    pub ids: Vec<String>,
    pub market: Option<CountryCode>,
}

#[derive(Clone, Debug, Default, Deserialize)]
pub struct GetAlbumListResponse {
    pub albums: Vec<Album>,
}

#[derive(Clone, Debug, Default, Deserialize)]
pub struct GetTrackListRequest {
    pub id: String,
    pub limit: Option<u32>,
    pub offset: Option<u32>,
    pub market: Option<CountryCode>,
}
