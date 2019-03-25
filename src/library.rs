use crate::object::{ObjectType, PagingObject, PagingObjectWrapper, SavedAlbum, SavedTrack};
use crate::{generate_params, Client, CountryCode};
use reqwest;
use reqwest::header::CONTENT_TYPE;
use serde::de::DeserializeOwned;
use serde_json::json;

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct LibraryClient {
    access_token: String,
    refresh_token: String,
}

impl Client for LibraryClient {
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

impl LibraryClient {
    pub fn new(access_token: &str, refresh_token: &str) -> Self {
        LibraryClient {
            access_token: access_token.to_string(),
            refresh_token: refresh_token.to_string(),
        }
    }

    pub fn check_saved_albums(&mut self, album_ids: &mut Vec<&str>) -> Vec<bool> {
        self.check(ObjectType::Albums, album_ids)
    }

    pub fn check_saved_tracks(&mut self, track_ids: &mut Vec<&str>) -> Vec<bool> {
        self.check(ObjectType::Tracks, track_ids)
    }

    pub fn check(&mut self, object_type: ObjectType, ids: &mut Vec<&str>) -> Vec<bool> {
        let url = format!(
            "https://api.spotify.com/v1/me/{}/contains",
            object_type.to_string()
        );
        let mut is_saved_list = Vec::new();
        if ids.len() > 50 {
            let mut drained: Vec<&str> = ids.drain(..50).collect();
            is_saved_list.append(&mut self.check(object_type, &mut drained));
            is_saved_list.append(&mut self.check(object_type, ids));
        }
        let request = reqwest::Client::new()
            .get(&url)
            .query(&[("ids", ids.join(","))]);
        let mut response = self.send(request).unwrap();
        let mut bools: Vec<bool> = response.json().unwrap();
        is_saved_list.append(&mut bools);

        is_saved_list
    }

    pub fn get_saved_albums(
        &mut self,
        limit: Option<u32>,
        offset: Option<u32>,
        market: Option<CountryCode>,
    ) -> PagingObjectWrapper<SavedAlbum> {
        self.get(ObjectType::Albums, limit, offset, market)
    }

    pub fn get_saved_tracks(
        &mut self,
        limit: Option<u32>,
        offset: Option<u32>,
        market: Option<CountryCode>,
    ) -> PagingObjectWrapper<SavedTrack> {
        self.get(ObjectType::Tracks, limit, offset, market)
    }

    pub fn get<T: DeserializeOwned + Clone>(
        &mut self,
        object_type: ObjectType,
        limit: Option<u32>,
        offset: Option<u32>,
        market: Option<CountryCode>,
    ) -> PagingObjectWrapper<T> {
        let url = format!("https://api.spotify.com/v1/me/{}", object_type.to_string());
        let mut params = generate_params(limit, offset);
        let market = market.map_or("from_token".to_string(), |v| v.alpha2().to_string());
        params.push(("market", market));
        let request = reqwest::Client::new().get(&url).query(&params);
        let mut response = self.send(request).unwrap();
        let paging_object: PagingObject<T> = response.json().unwrap();

        PagingObjectWrapper::new(
            paging_object,
            &self.get_access_token(),
            &self.get_refresh_token(),
        )
    }

    pub fn remove_saves_albums(&mut self, album_ids: &mut Vec<&str>) {
        self.remove(ObjectType::Albums, album_ids);
    }

    pub fn remove_saved_tracks(&mut self, track_ids: &mut Vec<&str>) {
        self.remove(ObjectType::Tracks, track_ids);
    }

    pub fn remove(&mut self, object_type: ObjectType, ids: &mut Vec<&str>) {
        let url = format!("https://api.spotify.com/v1/me/{}", object_type.to_string());
        if ids.len() > 50 {
            let mut drained: Vec<&str> = ids.drain(..50).collect();
            self.remove(object_type, &mut drained);
            self.remove(object_type, ids);
        }
        let params = [("ids", ids.join(","))];
        let request = reqwest::Client::new()
            .delete(&url)
            .header(CONTENT_TYPE, "application/json")
            .query(&params);
        self.send(request).unwrap();
    }

    pub fn save_albums(&mut self, album_ids: &mut Vec<&str>) {
        self.save(ObjectType::Albums, album_ids);
    }

    pub fn save_tracks(&mut self, track_ids: &mut Vec<&str>) {
        self.save(ObjectType::Tracks, track_ids);
    }

    pub fn save(&mut self, object_type: ObjectType, ids: &mut Vec<&str>) {
        let url = format!("https://api.spotify.com/v1/me/{}", object_type.to_string());
        if ids.len() > 50 {
            let mut drained: Vec<&str> = ids.drain(..50).collect();
            self.save(object_type, &mut drained);
            self.save(object_type, ids);
        }
        let request = reqwest::Client::new()
            .put(&url)
            .header(CONTENT_TYPE, "application/json")
            .json(&json!(ids));
        self.send(request).unwrap();
    }
}
