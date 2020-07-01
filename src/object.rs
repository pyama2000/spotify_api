use std::error::Error;

use serde::{de::DeserializeOwned, Deserialize};

use crate::RequestClient;

async fn get_paging_object<T: DeserializeOwned>(
    url: &str,
    access_token: &str,
    refresh_token: &str,
) -> Result<T, Box<dyn Error>> {
    let mut client = RequestClient::new(access_token, refresh_token);
    let request = reqwest::Client::new().get(url);
    let response = client.send(request).await?.unwrap();

    Ok(response.json().await?)
}

#[derive(Clone, Debug, Default, Deserialize)]
pub struct PagingObject<T> {
    pub href: String,
    pub items: Vec<T>,
    pub limit: u32,
    pub next: Option<String>,
    pub offset: Option<u32>,
    pub previous: Option<String>,
    pub total: Option<u32>,
}

impl<T: DeserializeOwned + Clone> PagingObject<T> {
    pub async fn get_next(
        &self,
        access_token: &str,
        refresh_token: &str,
    ) -> Result<Option<PagingObject<T>>, Box<dyn Error>> {
        let object = if let Some(url) = &self.next {
            Some(get_paging_object(url, access_token, refresh_token).await?)
        } else {
            None
        };

        Ok(object)
    }

    pub async fn get_previous(
        &self,
        access_token: &str,
        refresh_token: &str,
    ) -> Result<Option<PagingObject<T>>, Box<dyn Error>> {
        let object = if let Some(url) = &self.previous {
            Some(get_paging_object(url, access_token, refresh_token).await?)
        } else {
            None
        };

        Ok(object)
    }

    pub fn get_items(&self) -> Vec<T> {
        self.items.clone()
    }

    pub async fn get_all_items(
        &self,
        access_token: &str,
        refresh_token: &str,
    ) -> Result<Vec<T>, Box<dyn Error>> {
        let mut items: Vec<T> = Vec::new();
        let mut previous = self.get_previous(access_token, refresh_token).await?;

        while let Some(p) = previous {
            let mut prev_items: Vec<T> = p.get_items();
            prev_items.reverse();
            items.append(&mut prev_items);
            previous = p.get_previous(access_token, refresh_token).await?;
        }

        items.reverse();
        let mut current_items: Vec<T> = self.get_items();
        items.append(&mut current_items);

        let mut next = self.get_next(access_token, refresh_token).await?;
        while let Some(n) = next {
            let mut next_items: Vec<T> = n.get_items();
            items.append(&mut next_items);
            next = n.get_next(access_token, refresh_token).await?;
        }

        Ok(items)
    }
}

#[derive(Clone, Debug, Default, Deserialize)]
pub struct Cursor {
    after: String,
}

#[derive(Clone, Debug, Default, Deserialize)]
pub struct CursorPagingObject<T> {
    pub href: String,
    pub items: Vec<T>,
    pub limit: u32,
    pub next: Option<String>,
    pub cursors: Cursor,
    pub total: Option<u32>,
}

impl<T: DeserializeOwned + Clone> CursorPagingObject<T> {
    pub async fn get_next(
        &self,
        access_token: &str,
        refresh_token: &str,
    ) -> Result<Option<CursorPagingObject<T>>, Box<dyn Error>> {
        let object = if let Some(url) = &self.next {
            Some(get_paging_object(url, access_token, refresh_token).await?)
        } else {
            None
        };

        Ok(object)
    }

    pub fn get_items(&self) -> Vec<T> {
        self.items.clone()
    }

    pub async fn get_all_items(
        &self,
        access_token: &str,
        refresh_token: &str,
    ) -> Result<Vec<T>, Box<dyn Error>> {
        let mut items = self.get_items();

        let mut next = self.get_next(access_token, refresh_token).await?;
        while let Some(n) = next {
            // let mut next_items = n.get_items();
            // items.append(&mut next_items);
            items.append(&mut n.get_items());
            next = n.get_next(access_token, refresh_token).await?;
        }

        Ok(items)
    }
}

// #[derive(Deserialize, Serialize, Clone, Debug, Default)]
// pub struct PagingObjectWrapper<T> {
//     pub paging_object: PagingObject<T>,
//     access_token: String,
//     refresh_token: String,
// }
// 
// impl<T: DeserializeOwned + Clone> PagingObjectWrapper<T> {
//     pub fn new(paging_object: PagingObject<T>, access_token: &str, refresh_token: &str) -> Self {
//         PagingObjectWrapper {
//             paging_object,
//             access_token: access_token.to_string(),
//             refresh_token: refresh_token.to_string(),
//         }
//     }
// 
//     fn get_access_token(&self) -> String {
//         self.access_token.to_string()
//     }
// 
//     fn get_refresh_token(&self) -> String {
//         self.refresh_token.to_string()
//     }
// 
//     pub fn get_next(&self) -> Option<PagingObjectWrapper<T>> {
//         let object = self
//             .paging_object
//             .get_next(&self.access_token, &self.refresh_token);
// 
//         if let Some(o) = object {
//             let wrapper =
//                 PagingObjectWrapper::new(o, &self.get_access_token(), &self.get_refresh_token());
// 
//             Some(wrapper)
//         } else {
//             None
//         }
//     }
// 
//     pub fn get_previous(&self) -> Option<PagingObjectWrapper<T>> {
//         let object = self
//             .paging_object
//             .get_previous(&self.access_token, &self.refresh_token);
// 
//         if let Some(o) = object {
//             let wrapper =
//                 PagingObjectWrapper::new(o, &self.get_access_token(), &self.get_refresh_token());
// 
//             Some(wrapper)
//         } else {
//             None
//         }
//     }
// 
//     pub fn get_items(&self) -> Vec<T> {
//         self.paging_object.get_items()
//     }
// 
//     pub fn get_all_items(&self) -> Vec<T> {
//         self.paging_object
//             .get_all_items(&self.get_access_token(), &self.get_refresh_token())
//     }
// }

// use crate::Client;
// use chrono::serde::ts_milliseconds::deserialize as from_millis_ts;
// use chrono::{DateTime, Utc};
// use serde::de::DeserializeOwned;
// use std::convert::From;
// use std::fmt;
//
//
// #[derive(Deserialize, Serialize, Clone, Debug, Default)]
// pub struct Album {
//     pub album_type: String,
//     pub artists: Vec<Artist>,
//     pub available_markets: Option<Vec<String>>,
//     pub copyrights: Option<Vec<Copyrights>>,
//     pub external_ids: Option<ExternalID>,
//     pub external_urls: ExternalURL,
//     pub genres: Option<Vec<String>>,
//     pub href: String,
//     pub id: String,
//     pub images: Vec<Image>,
//     pub label: Option<String>,
//     pub name: String,
//     pub popularity: Option<u32>,
//     pub release_date: String,
//     pub release_date_precision: String,
//     pub tracks: Option<PagingObject<Track>>,
//     #[serde(rename = "type")]
//     pub object_type: String,
//     pub uri: String,
// }
//
// #[derive(Deserialize, Serialize, Clone, Debug, Default)]
// pub struct Artist {
//     pub external_urls: ExternalURL,
//     pub followers: Option<Follower>,
//     pub genres: Option<Vec<String>>,
//     pub href: String,
//     pub id: String,
//     pub images: Option<Vec<Image>>,
//     pub name: String,
//     pub popularity: Option<u32>,
//     #[serde(rename = "type")]
//     pub object_type: String,
//     pub uri: String,
// }
//
// #[derive(Deserialize, Serialize, Clone, PartialEq, PartialOrd, Debug)]
// pub struct AudioAnalysis {
//     pub bars: Vec<TimeInterval>,
//     pub beats: Vec<TimeInterval>,
//     pub sections: Vec<Section>,
//     pub segments: Vec<Segment>,
//     pub tatums: Vec<TimeInterval>,
// }
//
// #[derive(Deserialize, Serialize, Clone, PartialEq, PartialOrd, Debug)]
// pub struct AudioFeature {
//     pub duration_ms: u32,
//     pub key: u32,
//     pub mode: u32,
//     pub time_signature: u32,
//     pub acousticness: f32,
//     pub danceability: f32,
//     pub energy: f32,
//     pub instrumentalness: f32,
//     pub liveness: f32,
//     pub loudness: f32,
//     pub speechiness: f32,
//     pub valence: f32,
//     pub tempo: f32,
//     pub id: String,
//     pub uri: String,
//     pub track_href: String,
//     pub analysis_url: String,
//     #[serde(rename = "type")]
//     pub object_type: String,
// }
//
// #[derive(Deserialize, Serialize, Clone, Debug, Default)]
// pub struct Category {
//     pub href: String,
//     pub icons: Vec<Image>,
//     pub id: String,
//     pub name: String,
// }
//
// #[derive(Deserialize, Serialize, Clone, Debug, Default)]
// pub struct Copyrights {}
//
// #[derive(Deserialize, Serialize, Clone, Debug, Default)]
// pub struct Context {
//     uri: String,
//     href: String,
//     external_urls: ExternalURL,
//     #[serde(rename = "type")]
//     object_type: String,
// }
//
// #[derive(Deserialize, Serialize, Clone, Debug)]
// pub struct CurrentlyPlayingContext {
//     pub device: Device,
//     pub repeat_state: String,
//     pub shuffle_state: bool,
//     pub context: Option<Context>,
//     #[serde(deserialize_with = "from_millis_ts")]
//     pub timestamp: DateTime<Utc>,
//     pub progress_ms: Option<u32>,
//     pub is_playing: bool,
//     pub item: Option<Track>,
//     pub currently_playing_type: String,
// }
//
// #[derive(Deserialize, Serialize, Clone, Debug, Default)]
// pub struct Cursor {
//     after: Option<String>,
// }
//
// #[derive(Deserialize, Serialize, Clone, Debug, Default)]
// pub struct Device {
//     pub id: Option<String>,
//     pub is_active: bool,
//     pub is_private_session: bool,
//     pub is_restricted: bool,
//     pub name: String,
//     #[serde(rename = "type")]
//     pub device_type: String,
//     pub volume_percent: Option<u32>,
// }
//
// impl Device {
//     #[allow(dead_code)]
//     fn get_device_type(&self) -> DeviceType {
//         let device_type = &self.device_type;
//         DeviceType::from(device_type.as_str())
//     }
// }
//
// #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
// pub enum DeviceType {
//     Computer,
//     Tablet,
//     Smartphone,
//     Speaker,
//     TV,
//     AVR,
//     STB,
//     AudioDongle,
//     GameConsole,
//     CastVideo,
//     CastAudio,
//     Automobile,
//     Unknown,
// }
//
// impl From<&str> for DeviceType {
//     fn from(value: &str) -> Self {
//         match value {
//             "Computer" => DeviceType::Computer,
//             "Tablet" => DeviceType::Tablet,
//             "Smartphone" => DeviceType::Smartphone,
//             "Speaker" => DeviceType::Speaker,
//             "TV" => DeviceType::TV,
//             "AVR" => DeviceType::AVR,
//             "STB" => DeviceType::STB,
//             "AudioDongle" => DeviceType::AudioDongle,
//             "GameConsole" => DeviceType::GameConsole,
//             "CastVideo" => DeviceType::CastVideo,
//             "CastAudio" => DeviceType::CastAudio,
//             "Automobile" => DeviceType::Automobile,
//             "Unknown" => DeviceType::Unknown,
//             &_ => DeviceType::Unknown,
//         }
//     }
// }
//
// #[derive(Deserialize, Serialize, Clone, Debug, Default)]
// pub struct ExternalID {}
//
// #[derive(Deserialize, Serialize, Clone, Debug, Default)]
// pub struct ExternalURL {}
//
// #[derive(Deserialize, Serialize, Clone, Debug, Default)]
// pub struct Follower {
//     pub href: Option<String>,
//     pub total: u32,
// }
//
// #[derive(Deserialize, Serialize, Clone, Debug, Default)]
// pub struct Image {
//     pub height: Option<u32>,
//     pub url: String,
//     pub width: Option<u32>,
// }
//
// #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
// pub enum ObjectType {
//     Albums,
//     Artist,
//     Artists,
//     Tracks,
//     User,
// }
//
// impl fmt::Display for ObjectType {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         match *self {
//             ObjectType::Albums => write!(f, "albums"),
//             ObjectType::Artist => write!(f, "artist"),
//             ObjectType::Artists => write!(f, "artists"),
//             ObjectType::Tracks => write!(f, "tracks"),
//             ObjectType::User => write!(f, "user"),
//         }
//     }
// }
//

//
// #[derive(Deserialize, Serialize, Clone, Debug, Default)]
// pub struct PlayHistory {
//     track: Track,
//     played_at: String,
//     context: Option<Context>,
// }
//
// #[derive(Deserialize, Serialize, Clone, Debug, Default)]
// pub struct Playlist {
//     pub collaborative: bool,
//     pub description: Option<String>,
//     pub external_urls: ExternalURL,
//     pub followers: Option<Follower>,
//     pub href: String,
//     pub id: String,
//     pub images: Vec<Image>,
//     pub name: String,
//     pub owner: User,
//     pub public: Option<bool>,
//     pub snapshot_id: String,
//     #[serde(rename = "type")]
//     pub object_type: String,
//     pub uri: String,
// }
//
// #[derive(Deserialize, Serialize, Clone, Debug)]
// pub struct PlaylistTrack {
//     pub added_at: DateTime<Utc>,
//     pub added_by: User,
//     pub is_local: bool,
//     pub track: Track,
// }
//
// #[derive(Deserialize, Serialize, Clone, Debug, Default)]
// pub struct RecommendationResponse {
//     pub seeds: Vec<RecommendationSeed>,
//     pub tracks: Vec<Track>,
// }
//
// #[derive(Deserialize, Serialize, Clone, Debug, Default)]
// pub struct RecommendationSeed {
//     #[serde(rename = "afterFilteringSize")]
//     pub after_filtering_size: u32,
//     #[serde(rename = "afterRelinkingSize")]
//     pub after_relinking_size: u32,
//     pub href: String,
//     pub id: String,
//     #[serde(rename = "initialPoolSize")]
//     pub initial_pool_size: u32,
//     #[serde(rename = "type")]
//     seed_type: String,
// }
//
// #[derive(Deserialize, Serialize, Clone, Debug, Default)]
// pub struct SavedAlbum {
//     added_at: Option<DateTime<Utc>>,
//     album: Album,
// }
//
// #[derive(Deserialize, Serialize, Clone, Debug, Default)]
// pub struct SavedTrack {
//     added_at: Option<DateTime<Utc>>,
//     track: Track,
// }
//
// #[derive(Deserialize, Serialize, Clone, PartialEq, PartialOrd, Debug)]
// pub struct Section {
//     #[serde(flatten)]
//     pub time_interval: TimeInterval,
//     pub loudness: f64,
//     pub tempo: f64,
//     pub tempo_confidence: f64,
//     pub key: u64,
//     pub key_confidence: f64,
//     pub mode: u64,
//     pub mode_confidence: f64,
//     pub time_signature: u64,
//     pub time_signature_confidence: f64,
// }
//
// #[derive(Deserialize, Serialize, Clone, PartialEq, PartialOrd, Debug)]
// pub struct Segment {
//     #[serde(flatten)]
//     pub time_interval: TimeInterval,
//     pub loudness_start: f64,
//     pub loudness_max: f64,
//     pub loudness_max_time: f64,
//     pub loudness_end: Option<f64>,
//     pub pitches: Vec<f64>,
//     pub timbre: Vec<f64>,
// }
//
// #[derive(Deserialize, Serialize, Clone, Debug, Default)]
// pub struct Snapshot {
//     #[serde(rename = "snapshot_id")]
//     pub id: String,
// }
//
// #[derive(Deserialize, Serialize, Clone, PartialEq, PartialOrd, Debug)]
// pub struct TimeInterval {
//     pub start: f64,
//     pub duration: f64,
//     pub confidence: f64,
// }
//
// #[derive(Deserialize, Serialize, Clone, Debug, Default)]
// pub struct Track {
//     pub album: Option<Album>,
//     pub artists: Vec<Artist>,
//     pub available_markets: Option<Vec<String>>,
//     pub disc_number: u32,
//     pub duration_ms: u32,
//     pub explicit: bool,
//     pub external_ids: Option<ExternalID>,
//     pub external_urls: ExternalURL,
//     pub href: String,
//     pub id: String,
//     pub is_playable: Option<bool>,
//     pub name: String,
//     pub popularity: Option<u32>,
//     pub preview_url: Option<String>,
//     pub track_number: u32,
//     #[serde(rename = "type")]
//     pub object_type: String,
//     pub uri: String,
//     pub is_local: bool,
// }
//
// #[derive(Deserialize, Serialize, Clone, Debug, Default)]
// pub struct User {
//     pub birthdate: Option<String>,
//     pub country: Option<String>,
//     pub email: Option<String>,
//     pub display_name: Option<String>,
//     pub external_urls: ExternalURL,
//     pub followers: Option<Follower>,
//     pub href: String,
//     pub id: String,
//     pub images: Option<Vec<Image>>,
//     pub product: Option<String>,
//     #[serde(rename = "type")]
//     pub object_type: String,
//     pub uri: String,
// }
