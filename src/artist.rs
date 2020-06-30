use std::error::Error;

use futures::future::{BoxFuture, FutureExt};
use serde::Deserialize;

use crate::RequestClient;

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

// use crate::object::{Album, Artist, PagingObject, PagingObjectWrapper, Track};
// use crate::{generate_params, get_values, Client, CountryCode};
// use reqwest;
// use std::fmt;
//
// #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
// pub enum AlbumType {
//     Album,
//     Single,
//     AppearsOn,
//     Compilation,
// }
//
// impl fmt::Display for AlbumType {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         match *self {
//             AlbumType::Album => write!(f, "album"),
//             AlbumType::Single => write!(f, "single"),
//             AlbumType::AppearsOn => write!(f, "appears_on"),
//             AlbumType::Compilation => write!(f, "compilation"),
//         }
//     }
// }
//
// #[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
// pub struct ArtistClient {
//     access_token: String,
//     refresh_token: String,
// }
//
// impl Client for ArtistClient {
//     fn get_access_token(&self) -> String {
//         self.access_token.to_string()
//     }
//
//     fn get_refresh_token(&self) -> String {
//         self.refresh_token.to_string()
//     }
//
//     fn set_access_token(&mut self, access_token: &str) -> &mut Client {
//         self.access_token = access_token.to_string();
//         self
//     }
// }
//
// impl ArtistClient {
//     pub fn new(access_token: &str, refresh_token: &str) -> Self {
//         ArtistClient {
//             access_token: access_token.to_string(),
//             refresh_token: refresh_token.to_string(),
//         }
//     }
//
//     pub fn get_artist(&mut self, artist_id: &str) -> Artist {
//         let url = format!("https://api.spotify.com/v1/artists/{}", artist_id);
//         let request = reqwest::Client::new().get(&url);
//         let mut response = self.send(request).unwrap();
//
//         response.json().unwrap()
//     }
//
//     pub fn get_artists(&mut self, ids: &mut Vec<&str>) -> Vec<Artist> {
//         let mut artists = Vec::new();
//         if ids.len() > 50 {
//             let mut drained: Vec<&str> = ids.drain(..50).collect();
//             artists.append(&mut self.get_artists(&mut drained));
//             artists.append(&mut self.get_artists(ids));
//         }
//         let params = [("ids", ids.join(","))];
//         let request = reqwest::Client::new()
//             .get("https://api.spotify.com/v1/artists")
//             .query(&params);
//         let mut response = self.send(request).unwrap();
//         let mut objects: Vec<Artist> = get_values(&response.text().unwrap(), "artists").unwrap();
//         artists.append(&mut objects);
//
//         artists
//     }
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
