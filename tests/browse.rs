extern crate spotify_api;

#[cfg(test)]
mod browse {
    use spotify_api::browse::*;

    #[tokio::test]
    async fn get_category() {
        dotenv::dotenv().ok();
    
        let access_token = std::env::var("ACCESS_TOKEN").unwrap();
        let refresh_token = std::env::var("REFRESH_TOKEN").unwrap();
    
        let mut client = BrowseClient::new(&access_token, &refresh_token);
        let request = GetCategoryRequest {
            id: "party".to_string(),
            ..Default::default()
        };
        let category = client.get_category(request).await;
    
        assert_eq!("Party", category.unwrap().name);
    }
    
    #[tokio::test]
    async fn get_category_playlists() {
        dotenv::dotenv().ok();
    
        let access_token = std::env::var("ACCESS_TOKEN").unwrap();
        let refresh_token = std::env::var("REFRESH_TOKEN").unwrap();
    
        let mut client = BrowseClient::new(&access_token, &refresh_token);
        let request = GetCategoryPlaylistRequest {
            id: "party".to_string(),
            limit: Some(2),
            ..Default::default()
        };
        let playlists = client.get_category_playlists(request)
            .await
            .unwrap()
            .playlists
            .get_items();

        assert_eq!(2, playlists.len());
    }
    
    #[tokio::test]
    async fn get_categories() {
        dotenv::dotenv().ok();
    
        let access_token = std::env::var("ACCESS_TOKEN").unwrap();
        let refresh_token = std::env::var("REFRESH_TOKEN").unwrap();
    
        let mut client = BrowseClient::new(&access_token, &refresh_token);
        let request = GetCategoriesRequest {
            ..Default::default()
        };
        let _ = client.get_categories(request).await.unwrap();
    }

    #[tokio::test]
    async fn get_featured_playlists() {
        dotenv::dotenv().ok();
    
        let access_token = std::env::var("ACCESS_TOKEN").unwrap();
        let refresh_token = std::env::var("REFRESH_TOKEN").unwrap();
    
        let mut client = BrowseClient::new(&access_token, &refresh_token);
        let request = GetFeaturedPlaylistRequest {
            ..Default::default()
        };
        let _ = client.get_featured_playlists(request).await.unwrap();
    }

    #[tokio::test]
    async fn get_new_releases() {
        dotenv::dotenv().ok();
    
        let access_token = std::env::var("ACCESS_TOKEN").unwrap();
        let refresh_token = std::env::var("REFRESH_TOKEN").unwrap();
    
        let mut client = BrowseClient::new(&access_token, &refresh_token);
        let request = GetNewReleaseRequest {
            ..Default::default()
        };
        let _ = client.get_new_releases(request).await.unwrap();
    }

    #[tokio::test]
    async fn get_recommendations() {
        dotenv::dotenv().ok();
    
        let access_token = std::env::var("ACCESS_TOKEN").unwrap();
        let refresh_token = std::env::var("REFRESH_TOKEN").unwrap();
    
        let mut client = BrowseClient::new(&access_token, &refresh_token);
        let seed_artists = vec!["4NHQUGzhtTLFvgF5SZesLK".to_string()];
        let seed_tracks = vec!["0c6xIDDpzE81m2q797ordA".to_string()];
        let min_attributes = vec![
            TrackAttribute::Energy(0.4),
            TrackAttribute::Popularity(50),
        ];
        let request = GetRecommendationsRequest {
            seed_artists: Some(seed_artists),
            seed_tracks: Some(seed_tracks),
            min_attributes: Some(min_attributes),
            ..Default::default()
        };
        let _ = client.get_recommendations(request).await.unwrap();
    }
}
