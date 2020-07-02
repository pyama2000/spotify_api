extern crate spotify_api;
#[cfg(test)]
mod artist {
    use spotify_api::artist::*;
    
    #[tokio::test]
    async fn get_artist() {
        dotenv::dotenv().ok();
    
        let access_token = std::env::var("ACCESS_TOKEN").unwrap();
        let refresh_token = std::env::var("REFRESH_TOKEN").unwrap();
    
        let mut client = ArtistClient::new(&access_token, &refresh_token);
        let request = GetArtistRequest {
            id: "0OdUWJ0sBjDrqHygGUXeCF".to_string(),
        };
        let artist = client.get_artist(request).await;
    
        assert_eq!("Band of Horses".to_string(), artist.unwrap().name);
    }
    
    #[tokio::test]
    async fn get_artists() {
        dotenv::dotenv().ok();
    
        let access_token = std::env::var("ACCESS_TOKEN").unwrap();
        let refresh_token = std::env::var("REFRESH_TOKEN").unwrap();
    
        let mut client = ArtistClient::new(&access_token, &refresh_token);
        let ids = vec!["0oSGxfWSnnOXhD2fKuz2Gy", "3dBVyJ7JuOMt4GE9607Qin"]
            .into_iter()
            .map(|s| s.to_string())
            .collect();
        let request = GetArtistListRequest {
            ids,
        };
        let response = client.get_artists(request).await.unwrap();
    
        assert_eq!(2, response.artists.len());
    }
    
    #[tokio::test]
    async fn get_albums() {
        dotenv::dotenv().ok();
    
        let access_token = std::env::var("ACCESS_TOKEN").unwrap();
        let refresh_token = std::env::var("REFRESH_TOKEN").unwrap();
    
        let mut client = ArtistClient::new(&access_token, &refresh_token);
        let request = GetArtistAlbumRequest {
            id: "1vCWHaC5f2uS3yhpwWbIA6".to_string(),
            limit: Some(2),
            ..Default::default()
        };
        let albums = client.get_albums(request).await.unwrap().get_items();
    
        assert_eq!(2, albums.len());
    }
    
    #[tokio::test]
    async fn get_top_tracks() {
        dotenv::dotenv().ok();
    
        let access_token = std::env::var("ACCESS_TOKEN").unwrap();
        let refresh_token = std::env::var("REFRESH_TOKEN").unwrap();
    
        let mut client = ArtistClient::new(&access_token, &refresh_token);
        let request = GetArtistTopTrackRequest {
            id: "43ZHCT0cAZBISjO8DG9PnE".to_string(),
            ..Default::default()
        };
        let _ = client.get_top_tracks(request).await.unwrap();
    }
    
    #[tokio::test]
    async fn get_related_artists() {
        dotenv::dotenv().ok();
    
        let access_token = std::env::var("ACCESS_TOKEN").unwrap();
        let refresh_token = std::env::var("REFRESH_TOKEN").unwrap();
    
        let mut client = ArtistClient::new(&access_token, &refresh_token);
        let request = GetRelatedArtistRequest {
            id: "43ZHCT0cAZBISjO8DG9PnE".to_string(),
        };
        let _ = client.get_related_artists(request).await.unwrap();
    }
}
