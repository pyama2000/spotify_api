use self::start_playback::StartPlayback;
use crate::object::{
    CurrentlyPlayingContext, Device, PagingObject, PagingObjectWrapper, PlayHistory,
};
use crate::{get_values, Client, CountryCode};
use chrono::{DateTime, Utc};
use reqwest;
use reqwest::header::{HeaderMap, CONTENT_LENGTH, CONTENT_TYPE};
use reqwest::Method;
use serde::Deserialize;
use serde_json::json;
use std::fmt;

fn params_from_device_id(device_id: Option<String>) -> Vec<(&'static str, String)> {
    let mut params = Vec::new();
    if let Some(device_id) = device_id {
        params.push(("device_id", device_id));
    }
    params
}

fn request_from_device_id(
    method: Method,
    url: &str,
    device_id: Option<String>,
) -> reqwest::RequestBuilder {
    let mut request = reqwest::Client::new().request(method, url);
    if let Some(device_id) = device_id {
        request = request.query(&[("device_id", device_id)]);
    }
    request
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum TimeParameter {
    After(DateTime<Utc>),
    Before(DateTime<Utc>),
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum RepeatState {
    Track,
    Context,
    Off,
}

impl fmt::Display for RepeatState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RepeatState::Track => write!(f, "track"),
            RepeatState::Context => write!(f, "context"),
            RepeatState::Off => write!(f, "off"),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct CurrentlyPlayingTrackResponse {
    context: serde_json::Value,
    timestamp: u32,
    progress_ms: u32,
    is_playing: bool,
    item: serde_json::Value,
    currently_playing_type: String,
    actions: serde_json::Value,
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct PlayerClient {
    access_token: String,
    refresh_token: String,
}

impl Client for PlayerClient {
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

impl PlayerClient {
    pub fn new(access_token: &str, refresh_token: &str) -> Self {
        PlayerClient {
            access_token: access_token.to_string(),
            refresh_token: refresh_token.to_string(),
        }
    }

    pub fn get_devices(&mut self) -> Vec<Device> {
        let request = reqwest::Client::new().get("https://api.spotify.com/v1/me/player/devices");
        let mut response = self.send(request).unwrap();
        get_values(&response.text().unwrap(), "devices").unwrap()
    }

    pub fn get_current_playback(&mut self, market: Option<CountryCode>) -> CurrentlyPlayingContext {
        let market = market.map_or("from_token".to_string(), |v| v.alpha2().to_string());
        let request = reqwest::Client::new()
            .get("https://api.spotify.com/v1/me/player")
            .query(&[("market", market)]);
        let mut response = self.send(request).unwrap();
        response.json().unwrap()
    }

    pub fn get_currently_playing_track(&mut self, market: Option<CountryCode>) -> CurrentlyPlayingTrackResponse {
        let market = market.map_or("from_token".to_string(), |v| v.alpha2().to_string());
        let request = reqwest::Client::new()
            .get("https://api.spotify.com/v1/me/player/currently-playing")
            .query(&[("market", market)]);
        let mut response = self.send(request).unwrap();

        response.json().unwrap()
    }

    pub fn get_recently_played_track(
        &mut self,
        limit: Option<u32>,
        time_parameter: Option<TimeParameter>,
    ) -> PagingObjectWrapper<PlayHistory> {
        let limit = limit.filter(|&x| x <= 50).unwrap_or(20);
        let mut params = vec![("limit", i64::from(limit))];
        if let Some(timestamp) = time_parameter {
            match timestamp {
                TimeParameter::After(time) => {
                    let after = time.timestamp_millis();
                    params.push(("after", after));
                }
                TimeParameter::Before(time) => {
                    let before = time.timestamp_millis();
                    params.push(("before", before));
                }
            }
        }
        let request = reqwest::Client::new()
            .get("https://api.spotify.com/v1/me/player/recently-played")
            .query(&params);
        let mut response = self.send(request).unwrap();
        let paging_object: PagingObject<PlayHistory> = response.json().unwrap();

        PagingObjectWrapper::new(
            paging_object,
            &self.get_access_token(),
            &self.get_refresh_token(),
        )
    }

    pub fn pause(&mut self, device_id: Option<String>) {
        let mut request = request_from_device_id(
            Method::PUT,
            "https://api.spotify.com/v1/me/player/pause",
            device_id,
        );
        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());
        headers.insert(CONTENT_LENGTH, "0".parse().unwrap());
        request = request.headers(headers);
        self.send(request).unwrap();
    }

    pub fn seek_to_position(&mut self, position_ms: u64, device_id: Option<String>) {
        let mut params = params_from_device_id(device_id);
        params.push(("position_ms", position_ms.to_string()));
        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());
        headers.insert(CONTENT_LENGTH, "0".parse().unwrap());
        let request = reqwest::Client::new()
            .put("https://api.spotify.com/v1/me/player/seek")
            .headers(headers)
            .query(&params);
        self.send(request).unwrap();
    }

    pub fn set_repeat_mode(&mut self, state: RepeatState, device_id: Option<String>) {
        let mut params = params_from_device_id(device_id);
        params.push(("state", state.to_string()));
        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());
        headers.insert(CONTENT_LENGTH, "0".parse().unwrap());
        let request = reqwest::Client::new()
            .put("https://api.spotify.com/v1/me/player/repeat")
            .headers(headers)
            .query(&params);
        self.send(request).unwrap();
    }

    pub fn set_volume_percent(&mut self, volume_percent: u32, device_id: Option<String>) {
        let mut params = params_from_device_id(device_id);
        params.push(("volume_percent", volume_percent.to_string()));
        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());
        headers.insert(CONTENT_LENGTH, "0".parse().unwrap());
        let request = reqwest::Client::new()
            .put("https://api.spotify.com/v1/me/player/volume")
            .headers(headers)
            .query(&params);
        self.send(request).unwrap();
    }

    pub fn skip_next(&mut self, device_id: Option<String>) {
        self.skip("next", device_id);
    }

    pub fn skip_previous(&mut self, device_id: Option<String>) {
        self.skip("previous", device_id);
    }

    fn skip(&mut self, skip_to: &str, device_id: Option<String>) {
        let url = format!("https://api.spotify.com/v1/me/player/{}", skip_to);
        let mut request = request_from_device_id(Method::POST, &url, device_id);
        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());
        headers.insert(CONTENT_LENGTH, "0".parse().unwrap());
        request = request.headers(headers);
        self.send(request).unwrap();
    }

    pub fn play(&mut self, device_id: Option<String>, start_playback: Option<StartPlayback>) {
        let mut request = request_from_device_id(
            Method::PUT,
            "https://api.spotify.com/v1/me/player/play",
            device_id,
        );
        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());
        headers.insert(CONTENT_LENGTH, "0".parse().unwrap());
        request = request.headers(headers);
        if let Some(start_playback) = start_playback {
            request = request.json(&start_playback.json());
        }

        self.send(request).unwrap();
    }

    pub fn toggle_shuffle(&mut self, state: bool, device_id: Option<String>) {
        let mut params = params_from_device_id(device_id);
        params.push(("state", state.to_string()));
        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());
        headers.insert(CONTENT_LENGTH, "0".parse().unwrap());
        let request = reqwest::Client::new()
            .put("https://api.spotify.com/v1/me/player/shuffle")
            .headers(headers)
            .query(&params);
        self.send(request).unwrap();
    }

    pub fn transfer(&mut self, device_id: &str, play: Option<bool>) {
        let mut request = reqwest::Client::new()
            .put("https://api.spotify.com/v1/me/player")
            .header(CONTENT_TYPE, "application/json")
            .json(&json!({ "device_ids": [device_id] }));
        if let Some(play) = play {
            request = request.query(&[("play", play.to_string())]);
        }
        self.send(request).unwrap();
    }
}

pub mod start_playback {
    use serde_json::{json, Map, Value};

    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
    pub enum ContextType {
        Album,
        Artist,
        Playlist,
    }

    #[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
    pub enum Uri {
        Context {
            context_type: ContextType,
            uri: String,
        },
        Track {
            uris: Vec<String>,
        },
    }

    impl Uri {
        pub fn new_context(context_type: ContextType, uri: &str) -> Self {
            Uri::Context {
                context_type,
                uri: uri.to_string(),
            }
        }

        pub fn new_track(uris: Vec<String>) -> Self {
            Uri::Track { uris }
        }

        pub fn is_artist(&self) -> bool {
            match *self {
                Uri::Context { context_type, .. } => match context_type {
                    ContextType::Artist => true,
                    _ => false,
                },
                _ => false,
            }
        }
    }

    #[derive(Clone, Debug)]
    pub struct StartPlayback {
        uri: Uri,
        offset: Option<Value>,
        position_ms: Option<Value>,
    }

    impl StartPlayback {
        pub fn new(uri: Uri) -> Self {
            StartPlayback {
                uri,
                offset: None,
                position_ms: None,
            }
        }

        pub fn set_offset_from_position(&mut self, position: u32) {
            let offset = if !self.uri.is_artist() {
                let json = json!({
                    "position": position,
                });
                Some(json)
            } else {
                None
            };
            self.offset = offset;
        }

        pub fn set_offset_from_uri(&mut self, uri: &str) {
            let offset = if !self.uri.is_artist() {
                let json = json!({
                    "uri": uri,
                });
                Some(json)
            } else {
                None
            };
            self.offset = offset;
        }

        pub fn set_position_ms(&mut self, position_ms: u64) {
            self.position_ms = Some(json!(position_ms));
        }

        fn get_uri(&self) -> (String, Value) {
            match &self.uri {
                Uri::Context { uri, .. } => ("context_uri".to_string(), json!(uri)),
                Uri::Track { uris } => ("uris".to_string(), json!(uris)),
            }
        }

        pub fn json(self) -> Value {
            let mut map = Map::new();
            let uri = self.get_uri();
            map.insert(uri.0, uri.1);
            if let Some(offset) = self.offset {
                map.insert("offset".to_string(), offset);
            }
            if let Some(position_ms) = self.position_ms {
                map.insert("position_ms".to_string(), position_ms);
            }

            json!(map)
        }
    }
}
