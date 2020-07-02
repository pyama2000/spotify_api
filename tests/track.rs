extern crate spotify_api;

#[cfg(test)]
mod track {
    use spotify_api::track::*;

    #[tokio::test]
    async fn get_audio_analysis() {
        dotenv::dotenv().ok();
    
        let access_token = std::env::var("ACCESS_TOKEN").unwrap();
        let refresh_token = std::env::var("REFRESH_TOKEN").unwrap();
    
        let mut client = TrackClient::new(&access_token, &refresh_token);

        let _ = client.get_audio_analysis("3JIxjvbbDrA9ztYlNcp3yL").await.unwrap();
    }

    #[tokio::test]
    async fn get_audio_feature() {
        dotenv::dotenv().ok();
    
        let access_token = std::env::var("ACCESS_TOKEN").unwrap();
        let refresh_token = std::env::var("REFRESH_TOKEN").unwrap();
    
        let mut client = TrackClient::new(&access_token, &refresh_token);

        let _ = client.get_audio_feature("3JIxjvbbDrA9ztYlNcp3yL").await.unwrap();
    }

    #[tokio::test]
    async fn get_audio_features() {
        dotenv::dotenv().ok();
    
        let access_token = std::env::var("ACCESS_TOKEN").unwrap();
        let refresh_token = std::env::var("REFRESH_TOKEN").unwrap();
    
        let mut client = TrackClient::new(&access_token, &refresh_token);

        let ids = vec![
            "4JpKVNYnVcJ8tuMKjAj50A".to_string(),
            "2NRANZE9UCmPAS5XVbXL40".to_string(),
            "24JygzOLM0EmRQeGtFcIcG".to_string(),
        ];

        let _ = client.get_audio_features(ids).await.unwrap();
    }

    #[tokio::test]
    async fn get_track() {
        dotenv::dotenv().ok();
    
        let access_token = std::env::var("ACCESS_TOKEN").unwrap();
        let refresh_token = std::env::var("REFRESH_TOKEN").unwrap();
    
        let mut client = TrackClient::new(&access_token, &refresh_token);

        let track = client.get_track("11dFghVXANMlKmJXsNCbNl", None).await.unwrap();
        assert_eq!("Cut To The Feeling", &track.name);
    }

    #[tokio::test]
    async fn get_tracks() {
        dotenv::dotenv().ok();
    
        let access_token = std::env::var("ACCESS_TOKEN").unwrap();
        let refresh_token = std::env::var("REFRESH_TOKEN").unwrap();
    
        let mut client = TrackClient::new(&access_token, &refresh_token);

        let ids: Vec<String> = vec![
            "11dFghVXANMlKmJXsNCbNl",
            "20I6sIOMTCkB6w7ryavxtO",
            "7xGfFoTpQ2E7fRF5lN10tr",
        ].repeat(101).into_iter().map(|v| v.to_string()).collect();

        let tracks = client.get_tracks(ids.clone(), None).await.unwrap();

        assert_eq!(ids.len(), tracks.len());
    }
}
