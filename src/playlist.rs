use crate::object::{Image, PagingObject, PagingObjectWrapper, Playlist, PlaylistTrack, Snapshot};
use crate::{generate_params, Client, CountryCode};
use reqwest;
use reqwest::header::{HeaderMap, CONTENT_LENGTH, CONTENT_TYPE};
use serde_json::{json, Map, Value};

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct PlaylistClient {
    access_token: String,
    refresh_token: String,
}

impl Client for PlaylistClient {
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

impl PlaylistClient {
    pub fn new(access_token: &str, refresh_token: &str) -> Self {
        PlaylistClient {
            access_token: access_token.to_string(),
            refresh_token: refresh_token.to_string(),
        }
    }

    pub fn add_tracks(
        &mut self,
        playlist_id: &str,
        uris: &mut Vec<&str>,
        position: Option<u32>,
    ) -> Vec<Snapshot> {
        let url = format!(
            "https://api.spotify.com/v1/playlists/{}/tracks",
            playlist_id
        );
        let position = position.unwrap_or(0);
        let mut snapshots = Vec::new();
        if uris.len() > 100 {
            let mut drained: Vec<&str> = uris.drain(..100).collect();
            snapshots.append(&mut self.add_tracks(playlist_id, &mut drained, Some(position)));
            snapshots.append(&mut self.add_tracks(playlist_id, uris, Some(position + 100)));
        }
        let json = json!({
            "uris": uris,
            "position": position,
        });
        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());
        headers.insert(CONTENT_LENGTH, "0".parse().unwrap());
        let request = reqwest::Client::new()
            .post(&url)
            .headers(headers)
            .json(&json);
        let mut response = self.send(request).unwrap();
        let snapshot: Snapshot = response.json().unwrap();
        snapshots.push(snapshot);
        snapshots
    }

    pub fn change_details(
        &mut self,
        playlist_id: &str,
        name: Option<&str>,
        public: Option<bool>,
        collaborative: Option<bool>,
        description: Option<&str>,
    ) {
        let url = format!("https://api.spotify.com/v1/playlists/{}", playlist_id);
        let mut params = Map::new();
        if let Some(name) = name {
            params.insert("name".to_string(), json!(name));
        }
        if let Some(public) = public {
            params.insert("public".to_string(), json!(public));
        }
        if let Some(collaborative) = collaborative {
            params.insert("collaborative".to_string(), json!(collaborative));
        }
        if let Some(description) = description {
            params.insert("description".to_string(), json!(description));
        }
        let request = reqwest::Client::new()
            .put(&url)
            .header(CONTENT_TYPE, "application/json")
            .json(&json!(params));
        self.send(request).unwrap();
    }

    pub fn rename(&mut self, playlist_id: &str, name: &str) {
        self.change_details(playlist_id, Some(name), None, None, None);
    }

    pub fn change_public(&mut self, playlist_id: &str, public: bool) {
        self.change_details(playlist_id, None, Some(public), None, None);
    }

    pub fn change_collaborative(&mut self, playlist_id: &str, collaborative: bool) {
        self.change_details(playlist_id, None, None, Some(collaborative), None);
    }

    pub fn change_description(&mut self, playlist_id: &str, description: &str) {
        self.change_details(playlist_id, None, None, None, Some(description));
    }

    pub fn create_playlist(
        &mut self,
        user_id: &str,
        name: &str,
        public: Option<bool>,
        collaborative: Option<bool>,
        description: Option<&str>,
    ) -> Option<Playlist> {
        let url = format!("https://api.spotify.com/v1/users/{}/playlists", user_id);
        let mut params = Map::new();
        params.insert("name".to_string(), json!(name));
        if let Some(public) = public {
            params.insert("public".to_string(), json!(public));
        }
        if let Some(collaborative) = collaborative {
            params.insert("collaborative".to_string(), json!(collaborative));
        }
        if let Some(description) = description {
            params.insert("description".to_string(), json!(description));
        }
        let request = reqwest::Client::new()
            .post(&url)
            .header(CONTENT_TYPE, "application/json")
            .json(&json!(params));
        let mut response = self.send(request).unwrap();
        response.json().ok()
    }

    pub fn get_current_user_playlists(
        &mut self,
        limit: Option<u32>,
        offset: Option<u32>,
    ) -> PagingObjectWrapper<Playlist> {
        self.get_playlists(None, limit, offset)
    }

    pub fn get_user_playlists(
        &mut self,
        user_id: &str,
        limit: Option<u32>,
        offset: Option<u32>,
    ) -> PagingObjectWrapper<Playlist> {
        self.get_playlists(Some(user_id), limit, offset)
    }

    pub fn get_playlists(
        &mut self,
        user_id: Option<&str>,
        limit: Option<u32>,
        offset: Option<u32>,
    ) -> PagingObjectWrapper<Playlist> {
        let url = if let Some(user_id) = user_id {
            format!("https://api.spotify.com/v1/users/{}/playlists", user_id)
        } else {
            "https://api.spotify.com/v1/me/playlists".to_string()
        };
        let params = generate_params(limit, offset);
        let request = reqwest::Client::new().get(&url).query(&params);
        let mut response = self.send(request).unwrap();
        let paging_object: PagingObject<Playlist> = response.json().unwrap();
        PagingObjectWrapper::new(
            paging_object,
            &self.get_access_token(),
            &self.get_refresh_token(),
        )
    }

    pub fn get_playlist(&mut self, playlist_id: &str, market: Option<CountryCode>) -> Playlist {
        let url = format!("https://api.spotify.com/v1/playlists/{}", playlist_id);
        let market = market.map_or("from_token".to_string(), |v| v.alpha2().to_string());
        let request = reqwest::Client::new()
            .get(&url)
            .query(&[("market", market)]);
        let mut response = self.send(request).unwrap();
        response.json().unwrap()
    }

    pub fn get_cover_images(&mut self, playlist_id: &str) -> Vec<Image> {
        let url = format!(
            "https://api.spotify.com/v1/playlists/{}/images",
            playlist_id
        );
        let request = reqwest::Client::new().get(&url);
        let mut response = self.send(request).unwrap();
        response.json().unwrap()
    }

    pub fn get_tracks(
        &mut self,
        playlist_id: &str,
        limit: Option<u32>,
        offset: Option<u32>,
        market: Option<CountryCode>,
    ) -> PagingObjectWrapper<PlaylistTrack> {
        let url = format!(
            "https://api.spotify.com/v1/playlists/{}/tracks",
            playlist_id
        );
        let mut params = generate_params(limit, offset);
        let market = market.map_or("from_token".to_string(), |v| v.alpha2().to_string());
        params.push(("market", market));
        let request = reqwest::Client::new().get(&url).query(&params);
        let mut response = self.send(request).unwrap();
        let paging_object: PagingObject<PlaylistTrack> = response.json().unwrap();
        PagingObjectWrapper::new(
            paging_object,
            &self.get_access_token(),
            &self.get_refresh_token(),
        )
    }

    pub fn remove_tracks(&mut self, playlist_id: &str, tracks: &mut Vec<&str>) -> Vec<Snapshot> {
        let url = format!(
            "https://api.spotify.com/v1/playlists/{}/tracks",
            playlist_id
        );
        let mut snapshots = Vec::new();
        if tracks.len() > 100 {
            let mut drained: Vec<&str> = tracks.drain(..100).collect();
            snapshots.append(&mut self.remove_tracks(playlist_id, &mut drained));
            snapshots.append(&mut self.remove_tracks(playlist_id, tracks));
        }
        let tracks: Vec<Value> = tracks.iter_mut().map(|v| json!({ "uri": v })).collect();
        let json = json!({
            "tracks": tracks,
        });
        let request = reqwest::Client::new()
            .delete(&url)
            .header(CONTENT_TYPE, "application/json")
            .json(&json);
        let mut response = self.send(request).unwrap();
        let snapshot: Snapshot = response.json().unwrap();
        snapshots.push(snapshot);
        snapshots
    }

    pub fn reorder_tracks(
        &mut self,
        playlist_id: &str,
        range_start: u32,
        range_length: Option<u32>,
        insert_before: u32,
    ) -> Snapshot {
        let url = format!(
            "https://api.spotify.com/v1/playlists/{}/tracks",
            playlist_id
        );
        let json = json!({
            "range_start": range_start,
            "range_length": range_length.unwrap_or(1),
            "insert_before": insert_before,
        });
        let request = reqwest::Client::new()
            .put(&url)
            .header(CONTENT_TYPE, "application/json")
            .json(&json);
        let mut response = self.send(request).unwrap();
        response.json().unwrap()
    }

    pub fn replace_tracks(&mut self, playlist_id: &str, uris: &mut Vec<&str>) {
        let url = format!(
            "https://api.spotify.com/v1/playlists/{}/tracks",
            playlist_id
        );
        if uris.len() > 100 {
            let mut drained: Vec<&str> = uris.drain(..100).collect();
            self.replace_tracks(playlist_id, &mut drained);
            self.replace_tracks(playlist_id, uris);
        }
        let json = json!({
            "uris": uris,
        });
        let request = reqwest::Client::new()
            .put(&url)
            .header(CONTENT_TYPE, "application/json")
            .json(&json);
        self.send(request).unwrap();
    }
}
