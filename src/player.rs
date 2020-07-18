use std::error::Error;

use chrono::{DateTime, Utc};
use isocountry::CountryCode;
use reqwest::{
    header::{HeaderMap, CONTENT_LENGTH, CONTENT_TYPE},
    Method, StatusCode,
};
use serde::{de::DeserializeOwned, Deserialize};
use serde_json::json;

use crate::{
    object::CursorPagingObject,
    track::{SimpleTrack, Track},
    RequestClient,
};

#[derive(Clone, Debug, Default)]
pub struct PlayerClient {
    client: RequestClient,
}

impl PlayerClient {
    pub fn new(access_token: &str, refresh_token: &str) -> Self {
        PlayerClient {
            client: RequestClient::new(access_token, refresh_token),
        }
    }

    pub async fn add_item(&mut self, request: AddItemRequest) -> Result<(), Box<dyn Error>> {
        let mut query = vec![("uri", request.uri)];
        if let Some(device_id) = request.device_id {
            query.push(("device_id", device_id));
        }

        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_LENGTH, "0".parse().unwrap());

        let builder = reqwest::Client::new()
            .post("https://api.spotify.com/v1/me/player/queue")
            .headers(headers)
            .query(&query);
        self.client.send(builder).await?.unwrap();

        Ok(())
    }

    pub async fn get_devices(&mut self) -> Result<GetDevicesResponse, Box<dyn Error>> {
        let builder = reqwest::Client::new().get("https://api.spotify.com/v1/me/player/devices");
        let response = self.client.send(builder).await?.unwrap();

        Ok(response.json().await?)
    }

    pub async fn get_current_playback(
        &mut self,
        request: GetCurrentlyRequest,
    ) -> Result<Option<CurrentlyPlayingContext>, Box<dyn Error>> {
        self.get_current("https://api.spotify.com/v1/me/player", request)
            .await
    }

    pub async fn get_currently_playing_track(
        &mut self,
        request: GetCurrentlyRequest,) -> Result<Option<CurrentlyPlayingObject>, Box<dyn Error>> {
        self.get_current(
            "https://api.spotify.com/v1/me/player/currently-playing",
            request,
        )
        .await
    }

    async fn get_current<T>(
        &mut self,
        url: &str,
        request: GetCurrentlyRequest,
    ) -> Result<Option<T>, Box<dyn Error>>
    where
        T: DeserializeOwned,
    {
        let query = if let Some(additional_types) = request.additional_types {
            let s: Vec<String> = additional_types
                .into_iter()
                .map(|v| v.to_string())
                .collect();

            vec![("additional_types", s)]
        } else {
            Vec::new()
        };

        let builder = reqwest::Client::new().get(url).query(&query);

        let response = self
            .client
            .set_market(request.market)
            .send(builder)
            .await?
            .unwrap();

        match response.status() {
            StatusCode::OK => Ok(Some(response.json().await?)),
            _ => Ok(None),
        }
    }

    pub async fn get_recently_played_tracks(
        &mut self,
        request: GetRecentlyPlayedTracksRequest,
    ) -> Result<CursorPagingObject<PlayHistory>, Box<dyn Error>> {
        let mut query = Vec::new();

        if let Some(after) = request.after {
            query.push(("after", after));
        }

        if let Some(before) = request.before {
            query.push(("before", before));
        }

        let builder = reqwest::Client::new()
            .get("https://api.spotify.com/v1/me/player/recently-played")
            .query(&query);

        let response = self
            .client
            .set_limit(request.limit)
            .send(builder)
            .await?
            .unwrap();

        Ok(response.json().await?)
    }

    pub async fn pause(&mut self, request: PauseRequest) -> Result<(), Box<dyn Error>> {
        self.action(ActionType::Pause, None, request.device_id)
            .await
    }

    pub async fn seek_to_position(&mut self, request: SeekRequest) -> Result<(), Box<dyn Error>> {
        let query = vec![("position_ms", request.position_ms.to_string())];

        self.action(ActionType::Seek, Some(query), request.device_id)
            .await
    }

    pub async fn set_repeat_mode(
        &mut self,
        request: SetRepeatModeRequest,
    ) -> Result<(), Box<dyn Error>> {
        let query = vec![("state", request.state.to_string())];

        self.action(ActionType::SetRepeatMode, Some(query), request.device_id)
            .await
    }

    pub async fn set_volume(&mut self, request: SetVolumeRequest) -> Result<(), Box<dyn Error>> {
        let query = vec![("volume_percent", request.volume_percent.to_string())];

        self.action(ActionType::SetVolume, Some(query), request.device_id)
            .await
    }

    pub async fn skip_next(&mut self, request: SkipRequest) -> Result<(), Box<dyn Error>> {
        self.action(ActionType::SkipNext, None, request.device_id)
            .await
    }

    pub async fn skip_previous(&mut self, request: SkipRequest) -> Result<(), Box<dyn Error>> {
        self.action(ActionType::SkipPrevious, None, request.device_id)
            .await
    }

    pub async fn toggle_shuffle(
        &mut self,
        request: ToggleShuffleRequest,
    ) -> Result<(), Box<dyn Error>> {
        let query = vec![("state", request.state.to_string())];

        self.action(ActionType::ToggleShuffle, Some(query), request.device_id)
            .await
    }

    async fn action(
        &mut self,
        action_type: ActionType,
        query: Option<Vec<(&str, String)>>,
        device_id: Option<String>,
    ) -> Result<(), Box<dyn Error>> {
        let url = format!(
            "https://api.spotify.com/v1/me/player/{}",
            &action_type.to_string(),
        );

        let mut query = query.unwrap_or_default();
        if let Some(device_id) = device_id {
            query.push(("device_id", device_id));
        }

        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_LENGTH, "0".parse().unwrap());

        let builder = reqwest::Client::new()
            .request(action_type.to_method(), &url)
            .headers(headers)
            .query(&query);

        self.client.send(builder).await?.unwrap();

        Ok(())
    }

    pub async fn start(&mut self, request: Option<StartRequest>) -> Result<(), Box<dyn Error>> {
        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());
        headers.insert(CONTENT_LENGTH, "0".parse().unwrap());

        let mut builder = reqwest::Client::new()
            .put("https://api.spotify.com/v1/me/player/play")
            .headers(headers);

        if let Some(request) = request {
            if let Some(device_id) = request.device_id {
                builder = builder.query(&[("device_id", device_id)]);
            }

            let mut json = if let Some(context_uri) = request.context_uri {
                let mut map = serde_json::Map::new();
                map.insert("context_uri".to_string(), json!(context_uri));

                map
            } else if let Some(uris) = request.uris {
                let mut map = serde_json::Map::new();
                map.insert("uris".to_string(), json!(uris));

                map
            } else {
                serde_json::Map::new()
            };

            if let Some(offset) = request.offset {
                json.insert("offset".to_string(), json!(offset));
            }

            if let Some(position_ms) = request.position_ms {
                json.insert("position_ms".to_string(), json!(position_ms));
            }

            builder = builder.json(&json);
        }

        self.client.send(builder).await?.unwrap();

        Ok(())
    }

    pub async fn transfer_playlback(
        &mut self,
        request: TransferPlaybackRequest,
    ) -> Result<(), Box<dyn Error>> {
        let mut json = serde_json::Map::new();
        json.insert("device_ids".to_string(), json!([request.device_id]));

        if let Some(play) = request.play {
            json.insert("play".to_string(), json!(play));
        }

        let builder = reqwest::Client::new()
            .put("https://api.spotify.com/v1/me/player")
            .json(&json);

        self.client.send(builder).await?.unwrap();

        Ok(())
    }
}

#[derive(Clone, Debug, Default)]
pub struct AddItemRequest {
    pub uri: String,
    pub device_id: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize)]
pub struct GetDevicesResponse {
    pub devices: Vec<Device>,
}

#[derive(Clone, Debug, Default)]
pub struct GetCurrentlyRequest {
    pub market: Option<CountryCode>,
    pub additional_types: Option<Vec<ObjectType>>,
}

#[derive(Clone, Debug, Default)]
pub struct GetRecentlyPlayedTracksRequest {
    pub limit: Option<u32>,
    pub after: Option<usize>,
    pub before: Option<usize>,
}

#[derive(Clone, Debug, Default)]
pub struct PauseRequest {
    pub device_id: Option<String>,
}

#[derive(Clone, Debug, Default)]
pub struct SeekRequest {
    pub position_ms: usize,
    pub device_id: Option<String>,
}

#[derive(Clone, Debug, Default)]
pub struct SetRepeatModeRequest {
    pub state: RepeatState,
    pub device_id: Option<String>,
}

#[derive(Clone, Debug, Default)]
pub struct SetVolumeRequest {
    pub volume_percent: u32,
    pub device_id: Option<String>,
}

#[derive(Clone, Debug, Default)]
pub struct SkipRequest {
    pub device_id: Option<String>,
}

#[derive(Clone, Debug, Default)]
pub struct StartRequest {
    pub context_uri: Option<String>,
    pub uris: Option<Vec<String>>,
    pub offset: Option<serde_json::Value>,
    pub position_ms: Option<usize>,
    pub device_id: Option<String>,
}

#[derive(Clone, Debug, Default)]
pub struct TransferPlaybackRequest {
    pub device_id: String,
    pub play: Option<bool>,
}

#[derive(Clone, Debug, Default)]
pub struct ToggleShuffleRequest {
    pub state: bool,
    pub device_id: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize)]
pub struct CurrentlyPlayingContext {
    pub device: Device,
    pub repeat_state: RepeatState,
    pub shuffle_state: bool,
    #[serde(flatten)]
    pub object: CurrentlyPlayingObject,
    pub actions: serde_json::Value,
}

#[derive(Clone, Debug, Default, Deserialize)]
pub struct CurrentlyPlayingObject {
    pub context: Option<Context>,
    pub timestamp: usize,
    pub progress_ms: Option<u32>,
    pub is_playing: bool,
    pub item: Option<serde_json::Value>,
    pub currently_playing_type: ObjectType,
}

impl CurrentlyPlayingObject {
    pub fn get_track(&self) -> Option<Track> {
        match self.currently_playing_type {
            ObjectType::Track => serde_json::from_value(self.item.clone().unwrap()).ok(),
            _ => None,
        }
    }
}

#[derive(Clone, Debug, Default, Deserialize)]
pub struct Device {
    pub id: Option<String>,
    pub is_active: bool,
    pub is_private_session: bool,
    pub is_restricted: bool,
    pub name: String,
    #[serde(rename = "type")]
    pub device_type: DeviceType,
    pub volume_percent: Option<u32>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum DeviceType {
    Computer,
    Tablet,
    Smartphone,
    Speaker,
    TV,
    AVR,
    STB,
    AudioDongle,
    GameConsole,
    CastVideo,
    CastAudio,
    Automobile,
    Unknown,
}

impl Default for DeviceType {
    fn default() -> Self {
        DeviceType::Unknown
    }
}

#[derive(Clone, Debug, Default, Deserialize)]
pub struct Context {
    uri: String,
    href: Option<String>,
    #[serde(rename = "type")]
    pub device_type: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct PlayHistory {
    pub track: SimpleTrack,
    pub played_at: DateTime<Utc>,
    pub context: Context,
}

#[derive(Copy, Clone, Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RepeatState {
    Track,
    Context,
    Off,
}

impl std::fmt::Display for RepeatState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RepeatState::Track => write!(f, "track"),
            RepeatState::Context => write!(f, "context"),
            RepeatState::Off => write!(f, "off"),
        }
    }
}

impl Default for RepeatState {
    fn default() -> Self {
        RepeatState::Off
    }
}

#[derive(Copy, Clone, Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ObjectType {
    Episode,
    Track,
    Ad,
    Unknown,
}

impl std::fmt::Display for ObjectType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ObjectType::Episode => write!(f, "episode"),
            ObjectType::Track => write!(f, "track"),
            _ => write!(f, ""),
        }
    }
}

impl Default for ObjectType {
    fn default() -> Self {
        ObjectType::Unknown
    }
}

#[derive(Copy, Clone, Debug)]
pub enum ActionType {
    Pause,
    Seek,
    SetRepeatMode,
    SetVolume,
    SkipNext,
    SkipPrevious,
    ToggleShuffle,
}

impl std::fmt::Display for ActionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ActionType::Pause => write!(f, "pause"),
            ActionType::Seek => write!(f, "seek"),
            ActionType::SetRepeatMode => write!(f, "repeat"),
            ActionType::SetVolume => write!(f, "volume"),
            ActionType::SkipNext => write!(f, "next"),
            ActionType::SkipPrevious => write!(f, "previous"),
            ActionType::ToggleShuffle => write!(f, "shuffle"),
        }
    }
}

impl ActionType {
    pub fn to_method(self) -> Method {
        match self {
            ActionType::SkipNext | ActionType::SkipPrevious => Method::POST,
            ActionType::Pause
            | ActionType::Seek
            | ActionType::SetRepeatMode
            | ActionType::SetVolume => Method::PUT,
            ActionType::ToggleShuffle => Method::PUT,
        }
    }
}
