use serde::Deserialize;

use crate::{album::SimpleAlbum, artist::SimpleArtist};

#[derive(Clone, Debug, Default, Deserialize)]
pub struct Track {
    pub album: Option<SimpleAlbum>,
    pub artists: Vec<SimpleArtist>,
    pub available_markets: Option<Vec<String>>,
    pub disc_number: u32,
    pub duration_ms: u32,
    pub explicit: bool,
    // pub external_ids: Option<ExternalID>,
    // pub external_urls: ExternalURL,
    pub href: String,
    pub id: String,
    pub is_playable: Option<bool>,
    // pub linked_from: LinkedTrack,
    // pub restrictions: Restrictions
    pub name: String,
    pub popularity: Option<u32>,
    pub preview_url: Option<String>,
    pub track_number: u32,
    #[serde(rename = "type")]
    pub object_type: String,
    pub uri: String,
    pub is_local: bool,
}

#[derive(Clone, Debug, Default, Deserialize)]
pub struct SimpleTrack {
    // pub artists: Vec<Artist>,
    pub available_markets: Option<Vec<String>>,
    pub disc_number: u32,
    pub duration_ms: u32,
    pub explicit: bool,
    // pub external_ids: Option<ExternalID>,
    // pub external_urls: ExternalURL,
    pub href: String,
    pub id: String,
    pub is_playable: Option<bool>,
    // pub linked_from: LinkedTrack,
    pub name: String,
    pub preview_url: Option<String>,
    pub track_number: u32,
    #[serde(rename = "type")]
    pub object_type: String,
    pub uri: String,
}
