use crate::object::{Album, PagingObject, PagingObjectWrapper, Track};
use crate::{generate_params, get_values, Client, CountryCode};
use reqwest;

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct AlbumClient {
    access_token: String,
    refresh_token: String,
}

impl Client for AlbumClient {
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

impl AlbumClient {
    pub fn new(access_token: &str, refresh_token: &str) -> Self {
        AlbumClient {
            access_token: access_token.to_string(),
            refresh_token: refresh_token.to_string(),
        }
    }

    pub fn get_album(&mut self, album_id: &str, market: Option<CountryCode>) -> Album {
        let url = format!("https://api.spotify.com/v1/albums/{}", album_id);
        let market = market.map_or("from_token".to_string(), |v| v.alpha2().to_string());
        let request = reqwest::Client::new()
            .get(&url)
            .query(&[("market", market)]);
        let mut response = self.send(request).unwrap();

        response.json().unwrap()
    }

    pub fn get_albums(&mut self, ids: &mut Vec<&str>, market: Option<CountryCode>) -> Vec<Album> {
        let mut albums = Vec::new();
        if ids.len() > 20 {
            let mut drained: Vec<&str> = ids.drain(..20).collect();
            albums.append(&mut self.get_albums(&mut drained, market));
            albums.append(&mut self.get_albums(ids, market));
        }
        let market = market.map_or("from_token".to_string(), |v| v.alpha2().to_string());
        let params = [("ids", ids.join(",")), ("market", market)];
        let request = reqwest::Client::new()
            .get("https://api.spotify.com/v1/albums")
            .query(&params);
        let mut response = self.send(request).unwrap();
        let mut objects: Vec<Album> = get_values(&response.text().unwrap(), "albums").unwrap();
        albums.append(&mut objects);

        albums
    }

    pub fn get_tracks(
        &mut self,
        album_id: &str,
        limit: Option<u32>,
        offset: Option<u32>,
        market: Option<CountryCode>,
    ) -> PagingObjectWrapper<Track> {
        let url = format!("https://api.spotify.com/v1/albums/{}/tracks", album_id);
        let market = market.map_or("from_token".to_string(), |v| v.alpha2().to_string());
        let mut params = generate_params(limit, offset);
        params.push(("market", market));
        let request = reqwest::Client::new().get(&url).query(&params);
        let mut response = self.send(request).unwrap();

        let paging_object: PagingObject<Track> = response.json().unwrap();

        PagingObjectWrapper::new(
            paging_object,
            &self.get_access_token(),
            &self.get_refresh_token(),
        )
    }
}
