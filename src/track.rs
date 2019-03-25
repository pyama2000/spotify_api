use crate::object::{AudioAnalysis, AudioFeature, Track};
use crate::{get_values, Client, CountryCode};
use reqwest;

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct TrackClient {
    access_token: String,
    refresh_token: String,
}

impl Client for TrackClient {
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

impl TrackClient {
    pub fn new(access_token: &str, refresh_token: &str) -> Self {
        TrackClient {
            access_token: access_token.to_string(),
            refresh_token: refresh_token.to_string(),
        }
    }

    pub fn get_track(&mut self, id: &str, market: Option<CountryCode>) -> Track {
        let url = format!("https://api.spotify.com/v1/tracks/{}", id);
        let market = market.map_or("from_token".to_string(), |v| v.alpha2().to_string());
        let request = reqwest::Client::new()
            .get(&url)
            .query(&[("market", market)]);
        let mut response = self.send(request).unwrap();
        response.json().unwrap()
    }

    pub fn get_tracks(&mut self, ids: &mut Vec<&str>, market: Option<CountryCode>) -> Vec<Track> {
        let mut other_tracks = Vec::new();
        if ids.len() > 50 {
            let mut drained: Vec<&str> = ids.drain(..50).collect();
            other_tracks.append(&mut self.get_tracks(&mut drained, market));
            other_tracks.append(&mut self.get_tracks(ids, market));
        }
        let market = market.map_or("from_token".to_string(), |v| v.alpha2().to_string());
        let params = [("ids", ids.join(",")), ("market", market)];
        let request = reqwest::Client::new()
            .get("https://api.spotify.com/v1/tracks")
            .query(&params);
        let mut response = self.send(request).unwrap();
        let mut tracks: Vec<Track> = get_values(&response.text().unwrap(), "tracks").unwrap();
        other_tracks.append(&mut tracks);

        other_tracks
    }

    pub fn get_audio_analysis(&mut self, id: &str) -> AudioAnalysis {
        let url = format!("https://api.spotify.com/v1/audio-analysis/{}", id);
        let request = reqwest::Client::new().get(&url);
        let mut response = self.send(request).unwrap();
        response.json().unwrap()
    }

    pub fn get_audio_feature(&mut self, id: &str) -> AudioFeature {
        let url = format!("https://api.spotify.com/v1/audio-features/{}", id);
        let request = reqwest::Client::new().get(&url);
        let mut response = self.send(request).unwrap();
        response.json().unwrap()
    }

    pub fn get_audio_features(&mut self, ids: &mut Vec<&str>) -> Vec<AudioFeature> {
        let mut other_features = Vec::new();
        if ids.len() > 100 {
            let mut drained: Vec<&str> = ids.drain(..100).collect();
            other_features.append(&mut self.get_audio_features(&mut drained));
            other_features.append(&mut self.get_audio_features(ids));
        }
        let request = reqwest::Client::new()
            .get("https://api.spotify.com/v1/audio-features")
            .query(&[("ids", ids.join(","))]);
        let mut response = self.send(request).unwrap();
        let mut features: Vec<AudioFeature> =
            get_values(&response.text().unwrap(), "audio_features").unwrap();
        other_features.append(&mut features);

        other_features
    }
}
