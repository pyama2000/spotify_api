use crate::browse::recommendation::RecommendationFilter;
use crate::object::{
    Album, Category, PagingObject, PagingObjectWrapper, Playlist, RecommendationResponse,
};
use crate::{generate_params, get_value, Client, CountryCode};
use chrono::{DateTime, Utc};
use reqwest;
use std::collections::HashMap;

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct BrowseClient {
    access_token: String,
    refresh_token: String,
}

impl Client for BrowseClient {
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

impl BrowseClient {
    pub fn new(access_token: &str, refresh_token: &str) -> Self {
        BrowseClient {
            access_token: access_token.to_string(),
            refresh_token: refresh_token.to_string(),
        }
    }

    pub fn get_category(&mut self, category_id: &str, country: Option<CountryCode>) -> Category {
        let url = format!(
            "https://api.spotify.com/v1/browse/categories/{}",
            category_id
        );
        let params = if let Some(country) = country {
            vec![("country", country.alpha2())]
        } else {
            Vec::new()
        };
        let request = reqwest::Client::new().get(&url).query(&params);
        let mut response = self.send(request).unwrap();

        response.json().unwrap()
    }

    pub fn get_category_playlists(
        &mut self,
        category_id: &str,
        country: Option<CountryCode>,
        limit: Option<u32>,
        offset: Option<u32>,
    ) -> PagingObjectWrapper<Playlist> {
        let url = format!(
            "https://api.spotify.com/v1/browse/categories/{}/playlists",
            category_id
        );
        let mut params = generate_params(limit, offset);
        if let Some(country) = country {
            params.push(("country", country.alpha2().to_string()));
        }
        let request = reqwest::Client::new().get(&url).query(&params);
        let mut response = self.send(request).unwrap();
        let paging_object: PagingObject<Playlist> =
            get_value(&response.text().unwrap(), "playlists").unwrap();

        PagingObjectWrapper::new(
            paging_object,
            &self.get_access_token(),
            &self.get_refresh_token(),
        )
    }

    pub fn get_categories_list(
        &mut self,
        country: Option<CountryCode>,
        limit: Option<u32>,
        offset: Option<u32>,
    ) -> PagingObjectWrapper<Category> {
        let mut params = generate_params(limit, offset);
        if let Some(country) = country {
            params.push(("country", country.alpha2().to_string()));
        }
        let request = reqwest::Client::new()
            .get("https://api.spotify.com/v1/browse/categories")
            .query(&params);
        let mut response = self.send(request).unwrap();
        let paging_object: PagingObject<Category> =
            get_value(&response.text().unwrap(), "categories").unwrap();

        PagingObjectWrapper::new(
            paging_object,
            &self.get_access_token(),
            &self.get_refresh_token(),
        )
    }

    pub fn get_featured_playlists(
        &mut self,
        country: Option<CountryCode>,
        timestamp: Option<DateTime<Utc>>,
        limit: Option<u32>,
        offset: Option<u32>,
    ) -> (String, PagingObjectWrapper<Playlist>) {
        let mut params = generate_params(limit, offset);
        if let Some(timestamp) = timestamp {
            params.push((
                "timestamp",
                timestamp.format("%Y-%m-%dT%H:%M:%S").to_string(),
            ));
        }
        if let Some(country) = country {
            params.push(("country", country.alpha2().to_string()));
        }
        let request = reqwest::Client::new()
            .get("https://api.spotify.com/v1/browse/featured-playlists")
            .query(&params);
        let mut response = self.send(request).unwrap();
        let json_text = response.text().unwrap();
        let message: String = get_value(&json_text, "message").unwrap();
        let paging_object: PagingObject<Playlist> =
            get_value(&json_text, "playlists").unwrap();
        let wrapper = PagingObjectWrapper::new(
            paging_object,
            &self.get_access_token(),
            &self.get_refresh_token(),
        );

        (message, wrapper)
    }

    pub fn get_new_releases(
        &mut self,
        country: Option<CountryCode>,
        limit: Option<u32>,
        offset: Option<u32>,
    ) -> PagingObjectWrapper<Album> {
        let mut params = generate_params(limit, offset);
        if let Some(country) = country {
            params.push(("country", country.alpha2().to_string()));
        }
        let request = reqwest::Client::new()
            .get("https://api.spotify.com/v1/browse/new-releases")
            .query(&params);
        let mut response = self.send(request).unwrap();
        let paging_object: PagingObject<Album> =
            get_value(&response.text().unwrap(), "albums").unwrap();

        PagingObjectWrapper::new(
            paging_object,
            &self.get_access_token(),
            &self.get_refresh_token(),
        )
    }

    pub fn get_recommendations(
        &mut self,
        limit: Option<u32>,
        market: Option<CountryCode>,
        filter: RecommendationFilter,
    ) -> RecommendationResponse {
        let mut params = HashMap::new();
        let limit = limit.filter(|&x| x <= 100).unwrap_or(20);
        params.insert("limit".to_string(), limit.to_string());
        let market = market.map_or("from_token".to_string(), |v| v.alpha2().to_string());
        params.insert("market".to_string(), market);
        if !filter.max_attribute.is_empty() {
            for (k, v) in filter.max_attribute {
                params.insert(k, v);
            }
        }
        if !filter.min_attribute.is_empty() {
            for (k, v) in filter.min_attribute {
                params.insert(k, v);
            }
        }
        if !filter.seed.is_empty() {
            for (k, v) in filter.seed {
                params.insert(k, v);
            }
        }
        if !filter.target_attribute.is_empty() {
            for (k, v) in filter.target_attribute {
                params.insert(k, v);
            }
        }
        let request = reqwest::Client::new()
            .get("https://api.spotify.com/v1/recommendations")
            .query(&params);
        let mut resposne = self.send(request).unwrap();

        resposne.json().unwrap()
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
