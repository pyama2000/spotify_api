extern crate spotify_api;

#[cfg(test)]
mod player {
    use spotify_api::player::*;

    #[tokio::test]
    #[ignore]
    async fn add_item() {
        dotenv::dotenv().ok();
    
        let access_token = std::env::var("ACCESS_TOKEN").unwrap();
        let refresh_token = std::env::var("REFRESH_TOKEN").unwrap();
    
        let mut client = PlayerClient::new(&access_token, &refresh_token);

        let request = AddItemRequest {
            uri: "spotify:track:1301WleyT98MSxVHPZCA6M".to_string(),
            ..Default::default()
        };

        client.add_item(request).await.unwrap();
    }

    #[tokio::test]
    #[ignore]
    async fn get_devices() {
        dotenv::dotenv().ok();
    
        let access_token = std::env::var("ACCESS_TOKEN").unwrap();
        let refresh_token = std::env::var("REFRESH_TOKEN").unwrap();
    
        let mut client = PlayerClient::new(&access_token, &refresh_token);

        let devices = client.get_devices().await.unwrap().devices;
        dbg!(&devices);
    }

    #[tokio::test]
    #[ignore]
    async fn get_current_playback() {
        dotenv::dotenv().ok();
    
        let access_token = std::env::var("ACCESS_TOKEN").unwrap();
        let refresh_token = std::env::var("REFRESH_TOKEN").unwrap();
    
        let mut client = PlayerClient::new(&access_token, &refresh_token);
        let request = GetCurrentlyRequest {
            ..Default::default()
        };

        let context = client.get_current_playback(request).await.unwrap();
        if let Some(context) = context {
            dbg!(context.actions);
        }
    }

    #[tokio::test]
    #[ignore]
    async fn get_recently_tracks() {
        dotenv::dotenv().ok();
    
        let access_token = std::env::var("ACCESS_TOKEN").unwrap();
        let refresh_token = std::env::var("REFRESH_TOKEN").unwrap();
    
        let mut client = PlayerClient::new(&access_token, &refresh_token);
        let request = GetRecentlyPlayedTracksRequest {
            limit: Some(2),
            ..Default::default()
        };

        let histories = client.get_recently_played_tracks(request).await.unwrap().get_items();
        for history in histories {
            dbg!(history.track.name);
        }
    }

    #[tokio::test]
    #[ignore]
    async fn get_currently_playing_track() {
        dotenv::dotenv().ok();
    
        let access_token = std::env::var("ACCESS_TOKEN").unwrap();
        let refresh_token = std::env::var("REFRESH_TOKEN").unwrap();
    
        let mut client = PlayerClient::new(&access_token, &refresh_token);
        let request = GetCurrentlyRequest {
            ..Default::default()
        };

        let object = client.get_currently_playing_track(request).await.unwrap();
        if let Some(object) = object {
            dbg!(object.get_track().unwrap().name);
        }
    }

    #[tokio::test]
    #[ignore]
    async fn pause() {
        dotenv::dotenv().ok();
    
        let access_token = std::env::var("ACCESS_TOKEN").unwrap();
        let refresh_token = std::env::var("REFRESH_TOKEN").unwrap();
    
        let mut client = PlayerClient::new(&access_token, &refresh_token);
        let request = PauseRequest {
            ..Default::default()
        };

        client.pause(request).await.unwrap();
    }

    #[tokio::test]
    #[ignore]
    async fn resume() {
        dotenv::dotenv().ok();
    
        let access_token = std::env::var("ACCESS_TOKEN").unwrap();
        let refresh_token = std::env::var("REFRESH_TOKEN").unwrap();
    
        let mut client = PlayerClient::new(&access_token, &refresh_token);

        client.start(None).await.unwrap();
    }

    #[tokio::test]
    #[ignore]
    async fn seek_to_position() {
        dotenv::dotenv().ok();
    
        let access_token = std::env::var("ACCESS_TOKEN").unwrap();
        let refresh_token = std::env::var("REFRESH_TOKEN").unwrap();
    
        let mut client = PlayerClient::new(&access_token, &refresh_token);

        let request = SeekRequest {
            position_ms: 25000,
            ..Default::default()
        };

        client.seek_to_position(request).await.unwrap();
    }

    #[tokio::test]
    #[ignore]
    async fn set_repeat_mode() {
        dotenv::dotenv().ok();
    
        let access_token = std::env::var("ACCESS_TOKEN").unwrap();
        let refresh_token = std::env::var("REFRESH_TOKEN").unwrap();
    
        let mut client = PlayerClient::new(&access_token, &refresh_token);

        let request = SetRepeatModeRequest {
            state: RepeatState::Context,
            ..Default::default()
        };

        client.set_repeat_mode(request).await.unwrap();
    }

    #[tokio::test]
    #[ignore]
    async fn set_volume() {
        dotenv::dotenv().ok();
    
        let access_token = std::env::var("ACCESS_TOKEN").unwrap();
        let refresh_token = std::env::var("REFRESH_TOKEN").unwrap();
    
        let mut client = PlayerClient::new(&access_token, &refresh_token);

        let request = SetVolumeRequest {
            volume_percent: 20,
            ..Default::default()
        };

        client.set_volume(request).await.unwrap();
    }

    #[tokio::test]
    #[ignore]
    async fn skip_next() {
        dotenv::dotenv().ok();
    
        let access_token = std::env::var("ACCESS_TOKEN").unwrap();
        let refresh_token = std::env::var("REFRESH_TOKEN").unwrap();
    
        let mut client = PlayerClient::new(&access_token, &refresh_token);

        let request = SkipRequest {
            ..Default::default()
        };

        client.skip_next(request).await.unwrap();
    }

    #[tokio::test]
    #[ignore]
    async fn skip_previous() {
        dotenv::dotenv().ok();
    
        let access_token = std::env::var("ACCESS_TOKEN").unwrap();
        let refresh_token = std::env::var("REFRESH_TOKEN").unwrap();
    
        let mut client = PlayerClient::new(&access_token, &refresh_token);

        let request = SkipRequest {
            ..Default::default()
        };

        client.skip_previous(request).await.unwrap();
    }

    #[tokio::test]
    #[ignore]
    async fn start_context() {
        dotenv::dotenv().ok();
    
        let access_token = std::env::var("ACCESS_TOKEN").unwrap();
        let refresh_token = std::env::var("REFRESH_TOKEN").unwrap();
    
        let mut client = PlayerClient::new(&access_token, &refresh_token);

        let request = StartRequest {
            context_uri: Some("spotify:album:5ht7ItJgpBH7W6vJ5BqpPr".to_string()),
            ..Default::default()
        };

        client.start(Some(request)).await.unwrap();
    }

    #[tokio::test]
    #[ignore]
    async fn start_tracks() {
        dotenv::dotenv().ok();
    
        let access_token = std::env::var("ACCESS_TOKEN").unwrap();
        let refresh_token = std::env::var("REFRESH_TOKEN").unwrap();
    
        let mut client = PlayerClient::new(&access_token, &refresh_token);

        let request = StartRequest {
            uris: Some(vec!["spotify:track:1301WleyT98MSxVHPZCA6M".to_string()]),
            ..Default::default()
        };

        client.start(Some(request)).await.unwrap();
    }

    #[tokio::test]
    #[ignore]
    async fn toggle_shuffle() {
        dotenv::dotenv().ok();
    
        let access_token = std::env::var("ACCESS_TOKEN").unwrap();
        let refresh_token = std::env::var("REFRESH_TOKEN").unwrap();
    
        let mut client = PlayerClient::new(&access_token, &refresh_token);
        let request = ToggleShuffleRequest {
            state: true,
            ..Default::default()
        };

        client.toggle_shuffle(request).await.unwrap();
    }

    #[tokio::test]
    #[ignore]
    async fn transfer_playlback() {
        dotenv::dotenv().ok();
    
        let access_token = std::env::var("ACCESS_TOKEN").unwrap();
        let refresh_token = std::env::var("REFRESH_TOKEN").unwrap();
    
        let mut client = PlayerClient::new(&access_token, &refresh_token);
        let request = TransferPlaybackRequest {
            device_id: "".to_string(),
            play: Some(true),
        };

        client.transfer_playlback(request).await.unwrap();
    }
}
