use std::{error::Error, fmt};

use futures::future::{BoxFuture, FutureExt};
use isocountry::CountryCode;
use serde::Deserialize;

use crate::{album::SimpleAlbum, object::PagingObject, track::Track, RequestClient};

#[derive(Clone, Debug, Default, Deserialize)]
pub struct Artist {
    // pub external_urls: ExternalURL,
    // pub followers: Option<Follower>,
    pub genres: Option<Vec<String>>,
    pub href: String,
    pub id: String,
    // pub images: Option<Vec<Image>>,
    pub name: String,
    pub popularity: Option<u32>,
    #[serde(rename = "type")]
    pub object_type: String,
    pub uri: String,
}

#[derive(Clone, Debug, Default, Deserialize)]
pub struct SimpleArtist {
    // pub external_urls: ExternalURL,
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
        let mut query = Vec::new();

        let country = request
            .country
            .map_or("from_token".to_string(), |v| v.alpha2().to_string());
        query.push(("country", country));

        if let Some(groups) = request.include_groups {
            let s = groups
                .into_iter()
                .map(|s| s.to_string())
                .collect::<Vec<String>>()
                .join(",");
            query.push(("include_groups", s));
        }

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

// use crate::object::{Album, Artist, PagingObject, PagingObjectWrapper, Track};
// use crate::{generate_params, get_values, Client, CountryCode};
// use reqwest;
// use std::fmt;
//
//     pub fn get_albums(
//         &mut self,
//         artist_id: &str,
//         include_groups: Option<Vec<AlbumType>>,
//         country: Option<CountryCode>,
//         limit: Option<u32>,
//         offset: Option<u32>,
//     ) -> PagingObjectWrapper<Album> {
//         let url = format!("https://api.spotify.com/v1/artists/{}/albums", artist_id);
//         let country = country.map_or("from_token".to_string(), |v| v.alpha2().to_string());
//         let mut params = generate_params(limit, offset);
//         params.push(("country", country));
//         if let Some(mut groups) = include_groups {
//             groups.sort();
//             groups.dedup();
//             let groups_string = groups
//                 .iter()
//                 .map(std::string::ToString::to_string)
//                 .collect::<Vec<String>>()
//                 .join(",");
//             params.push(("include_groups", groups_string));
//         }
//         let request = reqwest::Client::new().get(&url).query(&params);
//         let mut response = self.send(request).unwrap();
//         let paging_object: PagingObject<Album> = response.json().unwrap();
//
//         PagingObjectWrapper::new(
//             paging_object,
//             &self.get_access_token(),
//             &self.get_refresh_token(),
//         )
//     }
//
//     pub fn get_top_tracks(&mut self, artist_id: &str, country: Option<CountryCode>) -> Vec<Track> {
//         let url = format!(
//             "https://api.spotify.com/v1/artists/{}/top-tracks",
//             artist_id
//         );
//         let country = country.map_or("from_token".to_string(), |v| v.alpha2().to_string());
//         let request = reqwest::Client::new()
//             .get(&url)
//             .query(&[("country", country)]);
//         let mut response = self.send(request).unwrap();
//
//         get_values(&response.text().unwrap(), "tracks").unwrap()
//     }
//
//     pub fn get_related_artists(&mut self, artist_id: &str) -> Vec<Artist> {
//         let url = format!(
//             "https://api.spotify.com/v1/artists/{}/related-artists",
//             artist_id
//         );
//         let request = reqwest::Client::new().get(&url);
//         let mut response = self.send(request).unwrap();
//
//         get_values(&response.text().unwrap(), "artists").unwrap()
//     }
// }
