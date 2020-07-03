use std::error::Error;

use futures::future::{BoxFuture, FutureExt};
use isocountry::CountryCode;
use serde::Deserialize;

use crate::{album::SimpleAlbum, artist::SimpleArtist, RequestClient};

#[derive(Clone, Debug, Default, Deserialize)]
pub struct Track {
    pub album: Option<SimpleAlbum>,
    pub artists: Vec<SimpleArtist>,
    pub available_markets: Option<Vec<String>>,
    pub disc_number: u32,
    pub duration_ms: u32,
    pub explicit: bool,
    pub href: String,
    pub id: String,
    pub is_playable: Option<bool>,
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
    pub artists: Vec<SimpleArtist>,
    pub available_markets: Option<Vec<String>>,
    pub disc_number: u32,
    pub duration_ms: u32,
    pub explicit: bool,
    pub href: String,
    pub id: String,
    pub is_playable: Option<bool>,
    pub name: String,
    pub preview_url: Option<String>,
    pub track_number: u32,
    #[serde(rename = "type")]
    pub object_type: String,
    pub uri: String,
}

#[derive(Clone, Debug, Default)]
pub struct TrackClient {
    client: RequestClient,
}

impl TrackClient {
    pub fn new(access_token: &str, refresh_token: &str) -> Self {
        TrackClient {
            client: RequestClient::new(access_token, refresh_token),
        }
    }

    pub async fn get_audio_analysis(
        &mut self,
        track_id: &str,
    ) -> Result<AudioAnalysis, Box<dyn Error>> {
        let url = format!("https://api.spotify.com/v1/audio-analysis/{}", track_id);
        let builder = reqwest::Client::new().get(&url);
        let response = self.client.send(builder).await?.unwrap();

        Ok(response.json().await?)
    }

    pub async fn get_audio_feature(
        &mut self,
        track_id: &str,
    ) -> Result<AudioFeature, Box<dyn Error>> {
        let url = format!("https://api.spotify.com/v1/audio-features/{}", track_id);
        let builder = reqwest::Client::new().get(&url);
        let response = self.client.send(builder).await?.unwrap();

        Ok(response.json().await?)
    }

    pub fn get_audio_features(
        &mut self,
        mut track_ids: Vec<String>,
    ) -> BoxFuture<'_, Result<Vec<AudioFeature>, Box<dyn Error>>> {
        async move {
            let mut features = Vec::new();
            if track_ids.len() > 100 {
                let drained = track_ids.drain(..100).collect();
                features.append(&mut self.get_audio_features(drained).await?);
                features.append(&mut self.get_audio_features(track_ids.clone()).await?);

                return Ok(features);
            }

            let builder = reqwest::Client::new()
                .get("https://api.spotify.com/v1/audio-features")
                .query(&[("ids", track_ids.join(","))]);

            let response = self.client.send(builder).await?.unwrap();
            let mut response: GetAudioFeaturesResponse = response.json().await?;

            features.append(&mut response.audio_features);

            Ok(features)
        }
        .boxed()
    }

    pub async fn get_track(
        &mut self,
        track_id: &str,
        market: Option<CountryCode>,
    ) -> Result<Track, Box<dyn Error>> {
        let url = format!("https://api.spotify.com/v1/tracks/{}", track_id);
        let builder = reqwest::Client::new().get(&url);
        let response = self.client.set_market(market).send(builder).await?.unwrap();

        Ok(response.json().await?)
    }

    pub fn get_tracks(
        &mut self,
        mut track_ids: Vec<String>,
        market: Option<CountryCode>,
    ) -> BoxFuture<'_, Result<Vec<Track>, Box<dyn Error>>> {
        async move {
            let mut tracks = Vec::new();
            if track_ids.len() > 50 {
                let drained = track_ids.drain(..50).collect();
                tracks.append(&mut self.get_tracks(drained, market).await?);
                tracks.append(&mut self.get_tracks(track_ids.clone(), market).await?);

                return Ok(tracks);
            }

            let builder = reqwest::Client::new()
                .get("https://api.spotify.com/v1/tracks")
                .query(&[("ids", track_ids.join(","))]);

            let response = self.client.set_market(market).send(builder).await?.unwrap();
            let mut results: GetTracksResponse = response.json().await?;

            tracks.append(&mut results.tracks);

            Ok(tracks)
        }
        .boxed()
    }
}

#[derive(Clone, Debug, Default, Deserialize)]
pub struct AudioAnalysis {
    pub bars: Vec<TimeInterval>,
    pub beats: Vec<TimeInterval>,
    pub sections: Vec<Section>,
    pub segments: Vec<Segment>,
    pub tatums: Vec<TimeInterval>,
}

#[derive(Clone, Debug, Default, Deserialize)]
pub struct TimeInterval {
    pub start: f64,
    pub duration: f64,
    pub confidence: f64,
}

#[derive(Clone, Debug, Default, Deserialize)]
pub struct Section {
    pub start: f64,
    pub duration: f64,
    pub confidence: f64,
    pub time_interval: TimeInterval,
    pub loudness: f64,
    pub tempo: f64,
    pub tempo_confidence: f64,
    pub key: u64,
    pub key_confidence: f64,
    pub mode: u64,
    pub mode_confidence: f64,
    pub time_signature: u64,
    pub time_signature_confidence: f64,
}

#[derive(Clone, Debug, Default, Deserialize)]
pub struct Segment {
    pub start: f64,
    pub duration: f64,
    pub confidence: f64,
    pub loudness_start: f64,
    pub loudness_max: f64,
    pub loudness_max_time: f64,
    pub loudness_end: Option<f64>,
    pub pitches: Vec<f64>,
    pub timbre: Vec<f64>,
}

#[derive(Clone, Debug, Default, Deserialize)]
pub struct AudioFeature {
    pub duration_ms: u32,
    pub key: u32,
    pub mode: u32,
    pub time_signature: u32,
    pub acousticness: f32,
    pub danceability: f32,
    pub energy: f32,
    pub instrumentalness: f32,
    pub liveness: f32,
    pub loudness: f32,
    pub speechiness: f32,
    pub valence: f32,
    pub tempo: f32,
    pub id: String,
    pub uri: String,
    pub track_href: String,
    pub analysis_url: String,
    #[serde(rename = "type")]
    pub object_type: String,
}

#[derive(Clone, Debug, Default, Deserialize)]
struct GetAudioFeaturesResponse {
    audio_features: Vec<AudioFeature>,
}

#[derive(Clone, Debug, Default, Deserialize)]
struct GetTracksResponse {
    tracks: Vec<Track>,
}
