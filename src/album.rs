use std::error::Error;

use isocountry::CountryCode;
use serde::Deserialize;

use crate::RequestClient;

#[derive(Deserialize, Clone, Debug)]
pub struct Album {
    pub album_type: String,
    pub available_markets: Option<Vec<String>>,
    pub genres: Vec<String>,
    pub href: String,
    pub id: String,
    pub label: String,
    pub name: String,
    pub popularity: u32,
    pub release_date: String,
    pub release_date_precision: String,
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

    pub async fn get_album(
        &mut self,
        request: GetAlbumRequest,
    ) -> Result<GetAlbumResponse, Box<dyn Error>> {
        let url = format!("https://api.spotify.com/v1/albums/{}", request.id);
        let market = request
            .market
            .map_or("from_token".to_string(), |v| v.alpha2().to_string());

        let builder = reqwest::Client::new()
            .get(&url)
            .query(&[("market", market)]);

        let response: GetAlbumResponse = self.client.send(builder).await?.unwrap().json().await?;

        Ok(response)
    }
}

#[derive(Debug, Deserialize)]
pub struct GetAlbumRequest {
    pub id: String,
    pub market: Option<CountryCode>,
}

#[derive(Debug, Deserialize)]
pub struct GetAlbumResponse {
    #[serde(flatten)]
    pub album: Album,
}
