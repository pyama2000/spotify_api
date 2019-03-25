use crate::object::{Album, Artist, PagingObject, PagingObjectWrapper, Track};
use crate::{generate_params, get_values, Client, CountryCode};
use reqwest;
use std::fmt;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum AlbumType {
    Album,
    Single,
    AppearsOn,
    Compilation,
}

impl fmt::Display for AlbumType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AlbumType::Album => write!(f, "album"),
            AlbumType::Single => write!(f, "single"),
            AlbumType::AppearsOn => write!(f, "appears_on"),
            AlbumType::Compilation => write!(f, "compilation"),
        }
    }
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct ArtistClient {
    access_token: String,
    refresh_token: String,
}

impl Client for ArtistClient {
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

impl ArtistClient {
    pub fn new(access_token: &str, refresh_token: &str) -> Self {
        ArtistClient {
            access_token: access_token.to_string(),
            refresh_token: refresh_token.to_string(),
        }
    }

    pub fn get_artist(&mut self, artist_id: &str) -> Artist {
        let url = format!("https://api.spotify.com/v1/artists/{}", artist_id);
        let request = reqwest::Client::new().get(&url);
        let mut response = self.send(request).unwrap();

        response.json().unwrap()
    }

    pub fn get_artists(&mut self, ids: &mut Vec<&str>) -> Vec<Artist> {
        let mut artists = Vec::new();
        if ids.len() > 50 {
            let mut drained: Vec<&str> = ids.drain(..50).collect();
            artists.append(&mut self.get_artists(&mut drained));
            artists.append(&mut self.get_artists(ids));
        }
        let params = [("ids", ids.join(","))];
        let request = reqwest::Client::new()
            .get("https://api.spotify.com/v1/artists")
            .query(&params);
        let mut response = self.send(request).unwrap();
        let mut objects: Vec<Artist> = get_values(&response.text().unwrap(), "artists").unwrap();
        artists.append(&mut objects);

        artists
    }

    pub fn get_albums(
        &mut self,
        artist_id: &str,
        include_groups: Option<Vec<AlbumType>>,
        country: Option<CountryCode>,
        limit: Option<u32>,
        offset: Option<u32>,
    ) -> PagingObjectWrapper<Album> {
        let url = format!("https://api.spotify.com/v1/artists/{}/albums", artist_id);
        let country = country.map_or("from_token".to_string(), |v| v.alpha2().to_string());
        let mut params = generate_params(limit, offset);
        params.push(("country", country));
        if let Some(mut groups) = include_groups {
            groups.sort();
            groups.dedup();
            let groups_string = groups
                .iter()
                .map(std::string::ToString::to_string)
                .collect::<Vec<String>>()
                .join(",");
            params.push(("include_groups", groups_string));
        }
        let request = reqwest::Client::new().get(&url).query(&params);
        let mut response = self.send(request).unwrap();
        let paging_object: PagingObject<Album> = response.json().unwrap();

        PagingObjectWrapper::new(
            paging_object,
            &self.get_access_token(),
            &self.get_refresh_token(),
        )
    }

    pub fn get_top_tracks(&mut self, artist_id: &str, country: Option<CountryCode>) -> Vec<Track> {
        let url = format!(
            "https://api.spotify.com/v1/artists/{}/top-tracks",
            artist_id
        );
        let country = country.map_or("from_token".to_string(), |v| v.alpha2().to_string());
        let request = reqwest::Client::new()
            .get(&url)
            .query(&[("country", country)]);
        let mut response = self.send(request).unwrap();

        get_values(&response.text().unwrap(), "tracks").unwrap()
    }

    pub fn get_related_artists(&mut self, artist_id: &str) -> Vec<Artist> {
        let url = format!(
            "https://api.spotify.com/v1/artists/{}/related-artists",
            artist_id
        );
        let request = reqwest::Client::new().get(&url);
        let mut response = self.send(request).unwrap();

        get_values(&response.text().unwrap(), "artists").unwrap()
    }
}
