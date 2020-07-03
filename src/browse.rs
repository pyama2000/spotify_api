use std::{error::Error, fmt};

use chrono::{DateTime, Utc};
use isocountry::CountryCode;
use serde::Deserialize;

use crate::{
    album::SimpleAlbum, object::{Image, PagingObject}, playlist::SimplePlaylist, track::SimpleTrack,
    RequestClient,
};

#[derive(Clone, Debug, Default, Deserialize)]
pub struct Category {
    pub href: String,
    pub icons: Vec<Image>,
    pub id: String,
    pub name: String,
}

#[derive(Clone, Debug, Default)]
pub struct BrowseClient {
    client: RequestClient,
}

impl BrowseClient {
    pub fn new(access_token: &str, refresh_token: &str) -> Self {
        BrowseClient {
            client: RequestClient::new(access_token, refresh_token),
        }
    }

    pub async fn get_category(
        &mut self,
        request: GetCategoryRequest,
    ) -> Result<Category, Box<dyn Error>> {
        let url = format!(
            "https://api.spotify.com/v1/browse/categories/{}",
            request.id
        );

        let mut query = Vec::new();
        if let Some(country) = request.country {
            query.push(("country", country.alpha2()));
        }
        if let Some(locale) = request.locale {
            query.push(("locale", locale.alpha2()));
        }

        let builder = reqwest::Client::new().get(&url).query(&query);
        let response = self.client.send(builder).await?.unwrap();

        Ok(response.json().await?)
    }

    pub async fn get_categories(
        &mut self,
        request: GetCategoriesRequest,
    ) -> Result<GetCategoriesResponse, Box<dyn Error>> {
        let mut query = Vec::new();
        if let Some(country) = request.country {
            query.push(("country", country.alpha2()));
        }
        if let Some(locale) = request.locale {
            query.push(("locale", locale.alpha2()));
        }

        let builder = reqwest::Client::new()
            .get("https://api.spotify.com/v1/browse/categories")
            .query(&query);

        let response = self
            .client
            .set_offset(request.offset)
            .set_limit(request.limit)
            .send(builder)
            .await?
            .unwrap();

        Ok(response.json().await?)
    }

    pub async fn get_category_playlists(
        &mut self,
        request: GetCategoryPlaylistRequest,
    ) -> Result<GetCategoryPlaylistResponse, Box<dyn Error>> {
        let url = format!(
            "https://api.spotify.com/v1/browse/categories/{}/playlists",
            request.id
        );

        let builder = reqwest::Client::new().get(&url);
        let response = self
            .client
            .set_offset(request.offset)
            .set_limit(request.limit)
            .set_country(request.country)
            .send(builder)
            .await?
            .unwrap();

        Ok(response.json().await?)
    }

    pub async fn get_featured_playlists(
        &mut self,
        request: GetFeaturedPlaylistRequest,
    ) -> Result<GetFeaturedPlaylistResponse, Box<dyn Error>> {
        let mut query = Vec::new();
        if let Some(country) = request.country {
            query.push(("country", country.alpha2().to_string()));
        }
        if let Some(locale) = request.locale {
            query.push(("locale", locale.alpha2().to_string()));
        }
        if let Some(timestamp) = request.timestamp {
            query.push((
                "timestamp",
                timestamp.format("%Y-%m-%dT%H:%M:%S").to_string(),
            ));
        }

        let builder = reqwest::Client::new()
            .get("https://api.spotify.com/v1/browse/featured-playlists")
            .query(&query);

        let response = self
            .client
            .set_offset(request.offset)
            .set_limit(request.limit)
            .set_country(request.country)
            .send(builder)
            .await?
            .unwrap();

        Ok(response.json().await?)
    }

    pub async fn get_new_releases(
        &mut self,
        request: GetNewReleaseRequest,
    ) -> Result<GetNewReleaseResponse, Box<dyn Error>> {
        let builder = reqwest::Client::new().get("https://api.spotify.com/v1/browse/new-releases");
        let response = self
            .client
            .set_offset(request.offset)
            .set_limit(request.limit)
            .set_country(request.country)
            .send(builder)
            .await?
            .unwrap();

        Ok(response.json().await?)
    }

    pub async fn get_recommendations(
        &mut self,
        request: GetRecommendationsRequest,
    ) -> Result<GetRecommendationsResponse, Box<dyn Error>> {
        let builder = reqwest::Client::new()
            .get("https://api.spotify.com/v1/recommendations")
            .query(&request.get_query());

        let resposne = self
            .client
            .set_limit(request.limit)
            .set_market(request.market)
            .send(builder)
            .await?
            .unwrap();

        Ok(resposne.json().await?)
    }
}

pub mod recommendation {
    use std::collections::HashMap;

    #[derive(Clone, Eq, PartialEq, Debug, Default)]
    pub struct RecommendationFilter {
        pub max_attribute: HashMap<String, String>,
        pub min_attribute: HashMap<String, String>,
        pub target_attribute: HashMap<String, String>,
        pub seed: Vec<(String, String)>,
    }

    impl RecommendationFilter {
        pub fn new() -> Self {
            RecommendationFilter {
                max_attribute: HashMap::new(),
                min_attribute: HashMap::new(),
                target_attribute: HashMap::new(),
                seed: Vec::new(),
            }
        }

        pub fn set_max_attribute(&mut self, attribute: TrackAttribute) -> &mut Self {
            let mut key = "max_".to_string();
            key.push_str(&attribute.get_name());
            self.max_attribute.insert(key, attribute.get_value());

            self
        }

        pub fn set_min_attribute(&mut self, attribute: TrackAttribute) -> &mut Self {
            let mut key = "min_".to_string();
            key.push_str(&attribute.get_name());
            self.min_attribute.insert(key, attribute.get_value());

            self
        }

        pub fn set_artist(&mut self, artist_id: &str) -> &mut Self {
            self.seed
                .push(("seed_artists".to_string(), artist_id.to_string()));
            self.seed.truncate(5);

            self
        }

        pub fn set_genre(&mut self, genre_id: &str) -> &mut Self {
            self.seed
                .push(("seed_genres".to_string(), genre_id.to_string()));
            self.seed.truncate(5);

            self
        }

        pub fn set_track(&mut self, track_id: &str) -> &mut Self {
            self.seed
                .push(("seed_tracks".to_string(), track_id.to_string()));
            self.seed.truncate(5);

            self
        }

        pub fn set_target_attribute(&mut self, attribute: TrackAttribute) -> &mut Self {
            let mut key = "target_".to_string();
            key.push_str(&attribute.get_name());
            self.target_attribute.insert(key, attribute.get_value());

            self
        }
    }

    #[derive(Copy, Clone, PartialEq, PartialOrd, Debug)]
    pub enum TrackAttribute {
        Acousticness(f32),
        Danceability(f32),
        DurationMs(i32),
        Energy(f32),
        Instrumentalness(f32),
        Key(i32),
        Liveness(f32),
        Loudness(f32),
        Mode(i32),
        Popularity(i32),
        Speechiness(f32),
        Tempo(f32),
        TimeSignature(i32),
        Valence(f32),
    }

    impl TrackAttribute {
        fn get_name(self) -> String {
            use self::TrackAttribute::*;

            match self {
                Acousticness(_) => "acousticness".to_string(),
                Danceability(_) => "danceability".to_string(),
                DurationMs(_) => "Duration_ms".to_string(),
                Energy(_) => "energy".to_string(),
                Instrumentalness(_) => "instrumentalness".to_string(),
                Key(_) => "key".to_string(),
                Liveness(_) => "liveness".to_string(),
                Loudness(_) => "loudness".to_string(),
                Mode(_) => "mode".to_string(),
                Popularity(_) => "popularity".to_string(),
                Speechiness(_) => "speechiness".to_string(),
                Tempo(_) => "tempo".to_string(),
                TimeSignature(_) => "time_signature".to_string(),
                Valence(_) => "valence".to_string(),
            }
        }

        fn get_value(self) -> String {
            use self::TrackAttribute::*;

            match self {
                Acousticness(v) => v.to_string(),
                Danceability(v) => v.to_string(),
                DurationMs(v) => v.to_string(),
                Energy(v) => v.to_string(),
                Instrumentalness(v) => v.to_string(),
                Key(v) => v.to_string(),
                Liveness(v) => v.to_string(),
                Loudness(v) => v.to_string(),
                Mode(v) => v.to_string(),
                Popularity(v) => v.to_string(),
                Speechiness(v) => v.to_string(),
                Tempo(v) => v.to_string(),
                TimeSignature(v) => v.to_string(),
                Valence(v) => v.to_string(),
            }
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct GetCategoryRequest {
    pub id: String,
    pub country: Option<CountryCode>,
    pub locale: Option<CountryCode>,
}

#[derive(Clone, Debug, Default)]
pub struct GetCategoryPlaylistRequest {
    pub id: String,
    pub country: Option<CountryCode>,
    pub offset: Option<u32>,
    pub limit: Option<u32>,
}

#[derive(Clone, Debug, Default, Deserialize)]
pub struct GetCategoryPlaylistResponse {
    pub playlists: PagingObject<SimplePlaylist>,
}

#[derive(Clone, Debug, Default)]
pub struct GetCategoriesRequest {
    pub country: Option<CountryCode>,
    pub locale: Option<CountryCode>,
    pub offset: Option<u32>,
    pub limit: Option<u32>,
}

#[derive(Clone, Debug, Default, Deserialize)]
pub struct GetCategoriesResponse {
    pub categories: PagingObject<Category>,
}

#[derive(Clone, Debug, Default)]
pub struct GetFeaturedPlaylistRequest {
    pub country: Option<CountryCode>,
    pub locale: Option<CountryCode>,
    pub timestamp: Option<DateTime<Utc>>,
    pub offset: Option<u32>,
    pub limit: Option<u32>,
}

#[derive(Clone, Debug, Default, Deserialize)]
pub struct GetFeaturedPlaylistResponse {
    pub message: String,
    pub playlists: PagingObject<SimplePlaylist>,
}

#[derive(Clone, Debug, Default)]
pub struct GetNewReleaseRequest {
    pub country: Option<CountryCode>,
    pub offset: Option<u32>,
    pub limit: Option<u32>,
}

#[derive(Clone, Debug, Default, Deserialize)]
pub struct GetNewReleaseResponse {
    pub albums: PagingObject<SimpleAlbum>,
}

#[derive(Clone, Debug, Default)]
pub struct GetRecommendationsRequest {
    pub limit: Option<u32>,
    pub market: Option<CountryCode>,
    pub seed_artists: Option<Vec<String>>,
    pub seed_genres: Option<Vec<String>>,
    pub seed_tracks: Option<Vec<String>>,
    pub max_attributes: Option<Vec<TrackAttribute>>,
    pub min_attributes: Option<Vec<TrackAttribute>>,
    pub target_attributes: Option<Vec<TrackAttribute>>,
}

impl GetRecommendationsRequest {
    fn get_query(&self) -> Vec<(String, String)> {
        let mut query = Vec::new();
        query.append(&mut self.get_seeds());
        query.append(&mut self.get_attribures());

        query
    }

    fn get_seeds(&self) -> Vec<(String, String)> {
        let mut seeds = Vec::new();

        if let Some(artists) = &self.seed_artists {
            let s = artists.join(",");
            seeds.push(("seed_artists".to_string(), s));
        }

        if let Some(genres) = &self.seed_genres {
            let s = genres.join(",");
            seeds.push(("seed_genres".to_string(), s));
        }

        if let Some(tracks) = &self.seed_tracks {
            let s = tracks.join(",");
            seeds.push(("seed_tracks".to_string(), s));
        }

        seeds
    }

    fn get_attribures(&self) -> Vec<(String, String)> {
        let mut attributes = Vec::new();

        if let Some(max_attributes) = &self.max_attributes {
            for attribute in max_attributes {
                let s = format!("max_{}", attribute.to_string());
                attributes.push((s, attribute.get_value()));
            }
        }

        if let Some(min_attributes) = &self.min_attributes {
            for attribute in min_attributes {
                let s = format!("min_{}", attribute.to_string());
                attributes.push((s, attribute.get_value()));
            }
        }

        if let Some(target_attributes) = &self.target_attributes {
            for attribute in target_attributes {
                let s = format!("target_{}", attribute.to_string());
                attributes.push((s, attribute.get_value()));
            }
        }

        attributes
    }
}

#[derive(Clone, Debug, Default, Deserialize)]
pub struct GetRecommendationsResponse {
    pub seeds: Vec<RecommendationSeed>,
    pub tracks: Vec<SimpleTrack>,
}

#[derive(Clone, Debug, Deserialize)]
pub enum TrackAttribute {
    Acousticness(f32),
    Danceability(f32),
    DurationMs(i32),
    Energy(f32),
    Instrumentalness(f32),
    Key(i32),
    Liveness(f32),
    Loudness(f32),
    Mode(i32),
    Popularity(i32),
    Speechiness(f32),
    Tempo(f32),
    TimeSignature(i32),
    Valence(f32),
}

impl TrackAttribute {
    pub fn get_value(&self) -> String {
        match &self {
            TrackAttribute::Acousticness(v) => v.to_string(),
            TrackAttribute::Danceability(v) => v.to_string(),
            TrackAttribute::DurationMs(v) => v.to_string(),
            TrackAttribute::Energy(v) => v.to_string(),
            TrackAttribute::Instrumentalness(v) => v.to_string(),
            TrackAttribute::Key(v) => v.to_string(),
            TrackAttribute::Liveness(v) => v.to_string(),
            TrackAttribute::Loudness(v) => v.to_string(),
            TrackAttribute::Mode(v) => v.to_string(),
            TrackAttribute::Popularity(v) => v.to_string(),
            TrackAttribute::Speechiness(v) => v.to_string(),
            TrackAttribute::Tempo(v) => v.to_string(),
            TrackAttribute::TimeSignature(v) => v.to_string(),
            TrackAttribute::Valence(v) => v.to_string(),
        }
    }
}

impl fmt::Display for TrackAttribute {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            TrackAttribute::Acousticness(_) => "acousticness",
            TrackAttribute::Danceability(_) => "danceability",
            TrackAttribute::DurationMs(_) => "Duration_ms",
            TrackAttribute::Energy(_) => "energy",
            TrackAttribute::Instrumentalness(_) => "instrumentalness",
            TrackAttribute::Key(_) => "key",
            TrackAttribute::Liveness(_) => "liveness",
            TrackAttribute::Loudness(_) => "loudness",
            TrackAttribute::Mode(_) => "mode",
            TrackAttribute::Popularity(_) => "popularity",
            TrackAttribute::Speechiness(_) => "speechiness",
            TrackAttribute::Tempo(_) => "tempo",
            TrackAttribute::TimeSignature(_) => "time_signature",
            TrackAttribute::Valence(_) => "valence",
        };

        write!(f, "{}", s.to_string())
    }
}

#[derive(Clone, Debug, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecommendationSeed {
    pub after_filtering_size: u32,
    pub after_relinking_size: u32,
    pub href: Option<String>,
    pub id: String,
    pub initial_pool_size: u32,
    #[serde(rename = "type")]
    pub object_type: String,
}
