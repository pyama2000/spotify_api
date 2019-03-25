use crate::object::{Artist, ObjectType, PagingObject, PagingObjectWrapper};
use crate::{get_value, Client};
use reqwest;
use reqwest::header::CONTENT_TYPE;
use serde_json::json;

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct FollowClient {
    access_token: String,
    refresh_token: String,
}

impl Client for FollowClient {
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

impl FollowClient {
    pub fn new(access_token: &str, refresh_token: &str) -> Self {
        FollowClient {
            access_token: access_token.to_string(),
            refresh_token: refresh_token.to_string(),
        }
    }

    pub fn check_following_artists(&mut self, artist_ids: &mut Vec<&str>) -> Vec<bool> {
        self.check_following(ObjectType::Artist, artist_ids)
    }

    pub fn check_following_users(&mut self, user_ids: &mut Vec<&str>) -> Vec<bool> {
        self.check_following(ObjectType::User, user_ids)
    }

    pub fn check_following(&mut self, object_type: ObjectType, ids: &mut Vec<&str>) -> Vec<bool> {
        let mut is_following_list = Vec::new();
        if ids.len() > 50 {
            let mut drained: Vec<&str> = ids.drain(..50).collect();
            is_following_list.append(&mut self.check_following(object_type, &mut drained));
            is_following_list.append(&mut self.check_following(object_type, ids));
        }
        let params = [("type", object_type.to_string()), ("ids", ids.join(","))];
        let request = reqwest::Client::new()
            .get("https://api.spotify.com/v1/me/following/contains")
            .query(&params);
        let mut response = self.send(request).unwrap();
        let mut list: Vec<bool> = response.json().unwrap();
        is_following_list.append(&mut list);

        is_following_list
    }

    pub fn check_users_following_playlist(
        &mut self,
        playlist_id: &str,
        user_ids: &mut Vec<&str>,
    ) -> Vec<bool> {
        let url = format!(
            "https://api.spotify.com/v1/playlists/{}/followers/contains",
            playlist_id
        );
        let mut list = Vec::new();
        if user_ids.len() > 5 {
            let mut drained: Vec<&str> = user_ids.drain(..5).collect(); 
            list.append(&mut self.check_users_following_playlist(playlist_id, &mut drained));
            list.append(&mut self.check_users_following_playlist(playlist_id, user_ids));
        }
        let params = [("ids", user_ids.join(","))];
        let request = reqwest::Client::new().get(&url).query(&params);
        let mut response = self.send(request).unwrap();
        let mut objects: Vec<bool> = response.json().unwrap();
        list.append(&mut objects);

        list
    }

    pub fn follow_artists(&mut self, artist_ids: &mut Vec<&str>) {
        self.follow(ObjectType::Artist, artist_ids);
    }

    pub fn follow_users(&mut self, user_ids: &mut Vec<&str>) {
        self.follow(ObjectType::User, user_ids);
    }

    pub fn follow(&mut self, object_type: ObjectType, ids: &mut Vec<&str>) {
        if ids.len() > 50 {
            let mut drained: Vec<&str> = ids.drain(..50).collect();
            self.follow(object_type, &mut drained);
            self.follow(object_type, ids);
        }
        let params = json!({
            "ids": ids,
        });
        let request = reqwest::Client::new()
            .put("https://api.spotify.com/v1/me/following")
            .header(CONTENT_TYPE, "application/json")
            .query(&[("type", object_type.to_string())])
            .json(&params);
        self.send(request).unwrap();
    }

    pub fn follow_playlist(&mut self, playlist_id: &str, public: bool) {
        let url = format!(
            "https://api.spotify.com/v1/playlists/{}/followers",
            playlist_id
        );
        let params = json!({
            "public": public,
        });
        let request = reqwest::Client::new()
            .put(&url)
            .header(CONTENT_TYPE, "application/json")
            .json(&params);
        self.send(request).unwrap();
    }

    pub fn get_followed_artists(
        &mut self,
        limit: Option<u32>,
        after: Option<String>,
    ) -> PagingObjectWrapper<Artist> {
        let mut params = vec![("type", "artist".to_string())];
        let limit = limit.filter(|&x| x <= 50).unwrap_or(20);
        params.push(("limit", limit.to_string()));
        if let Some(after) = after {
            params.push(("after", after));
        }
        let request = reqwest::Client::new()
            .get("https://api.spotify.com/v1/me/following")
            .query(&params);
        let mut response = self.send(request).unwrap();
        let paging_object: PagingObject<Artist> =
            get_value(&response.text().unwrap(), "artists").unwrap();

        PagingObjectWrapper::new(
            paging_object,
            &self.get_access_token(),
            &self.get_refresh_token(),
        )
    }

    pub fn unfollow_artists(&mut self, artist_ids: &mut Vec<&str>) {
        self.unfollow(ObjectType::Artist, artist_ids);
    }

    pub fn unfollow_users(&mut self, user_ids: &mut Vec<&str>) {
        self.unfollow(ObjectType::User, user_ids);
    }

    pub fn unfollow(&mut self, object_type: ObjectType, ids: &mut Vec<&str>) {
        if ids.len() > 50 {
            let mut drained: Vec<&str> = ids.drain(..50).collect();
            self.unfollow(object_type, &mut drained);
            self.unfollow(object_type, ids);
        }
        let params = [("type", object_type.to_string()), ("ids", ids.join(","))];
        let request = reqwest::Client::new()
            .delete("https://api.spotify.com/v1/me/following")
            .query(&params);
        self.send(request).unwrap();
    }

    pub fn unfollow_playlist(&mut self, playlist_id: &str) {
        let url = format!(
            "https://api.spotify.com/v1/playlists/{}/followers",
            playlist_id
        );
        let request = reqwest::Client::new().delete(&url);
        self.send(request).unwrap();
    }
}
