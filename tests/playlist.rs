extern crate spotify_api;

#[cfg(test)]
mod playlist {
    use spotify_api::playlist::*;

    #[tokio::test]
    // #[ignore]
    async fn add_items() {
        dotenv::dotenv().ok();

        let access_token = std::env::var("ACCESS_TOKEN").unwrap();
        let refresh_token = std::env::var("REFRESH_TOKEN").unwrap();

        let mut client = PlaylistClient::new(&access_token, &refresh_token);

        let mut uris = Vec::new();
        while uris.len() < 100 {
            uris.push("spotify:track:4iV5W9uYEdYUVa79Axb7Rh".to_string());
            uris.push("spotify:track:1301WleyT98MSxVHPZCA6M".to_string());
            uris.push("spotify:episode:512ojhOuo1ktJprKbVcKyQ".to_string());
        }
        dbg!(uris.len());

        let request = AddItemsRequest {
            playlist_id: "3PJeEBUHSVRa1rJ6KOP30H".to_string(),
            uris,
            ..Default::default()
        };

        let _ = client.add_items(request).await.unwrap();
    }

    #[tokio::test]
    #[ignore]
    async fn change_detail() {
        dotenv::dotenv().ok();

        let access_token = std::env::var("ACCESS_TOKEN").unwrap();
        let refresh_token = std::env::var("REFRESH_TOKEN").unwrap();

        let mut client = PlaylistClient::new(&access_token, &refresh_token);
        let request = ChangeDetailRequest {
            playlist_id: "3PJeEBUHSVRa1rJ6KOP30H".to_string(),
            name: Some("test".to_string()),
            public: Some(true),
            collaborative: Some(true),
            description: Some("test".to_string()),
        };

        client.change_detail(request).await.unwrap();
    }

    #[tokio::test]
    #[ignore]
    async fn create_playlist() {
        dotenv::dotenv().ok();

        let access_token = std::env::var("ACCESS_TOKEN").unwrap();
        let refresh_token = std::env::var("REFRESH_TOKEN").unwrap();

        let mut client = PlaylistClient::new(&access_token, &refresh_token);

        let request = CreatePlaylistRequest {
            user_id: "2v86jznkp2omgo6dor0y2y0yg".to_string(),
            name: "test".to_string(),
            description: Some("API test playlist".to_string()),
            ..Default::default()
        };

        let playlist = client.create_playlist(request).await.unwrap();
        dbg!(&playlist.id);
    }

    #[tokio::test]
    async fn get_playlists() {
        dotenv::dotenv().ok();

        let access_token = std::env::var("ACCESS_TOKEN").unwrap();
        let refresh_token = std::env::var("REFRESH_TOKEN").unwrap();

        let mut client = PlaylistClient::new(&access_token, &refresh_token);

        let request = GetPlaylistsRequest {
            limit: Some(2),
            ..Default::default()
        };

        let _ = client.get_playlists(request).await.unwrap().get_items();

        let request = GetPlaylistsRequest {
            user_id: Some("wizzler".to_string()),
            limit: Some(2),
            ..Default::default()
        };

        let _ = client.get_playlists(request).await.unwrap().get_items();
    }

    #[tokio::test]
    async fn get_playlist() {
        dotenv::dotenv().ok();

        let access_token = std::env::var("ACCESS_TOKEN").unwrap();
        let refresh_token = std::env::var("REFRESH_TOKEN").unwrap();

        let mut client = PlaylistClient::new(&access_token, &refresh_token);

        let request = GetPlaylistRequest {
            playlist_id: "3PJeEBUHSVRa1rJ6KOP30H".to_string(),
            ..Default::default()
        };

        let _ = client.get_playlist(request).await.unwrap();
    }

    #[tokio::test]
    async fn get_image() {
        dotenv::dotenv().ok();

        let access_token = std::env::var("ACCESS_TOKEN").unwrap();
        let refresh_token = std::env::var("REFRESH_TOKEN").unwrap();

        let mut client = PlaylistClient::new(&access_token, &refresh_token);

        let request = GetImageRequest {
            playlist_id: "3cEYpjA9oz9GiPac4AsH4n".to_string(),
        };

        let _ = client.get_image(request).await.unwrap();
    }

    #[tokio::test]
    async fn get_items() {
        dotenv::dotenv().ok();

        let access_token = std::env::var("ACCESS_TOKEN").unwrap();
        let refresh_token = std::env::var("REFRESH_TOKEN").unwrap();

        let mut client = PlaylistClient::new(&access_token, &refresh_token);

        let request = GetPlaylistTracksRequest {
            playlist_id: "3PJeEBUHSVRa1rJ6KOP30H".to_string(),
            limit: Some(2),
            ..Default::default()
        };

        let _ = client.get_tracks(request).await.unwrap().get_items();
    }

    #[tokio::test]
    #[ignore]
    async fn remove_items() {
        dotenv::dotenv().ok();

        let access_token = std::env::var("ACCESS_TOKEN").unwrap();
        let refresh_token = std::env::var("REFRESH_TOKEN").unwrap();

        let mut client = PlaylistClient::new(&access_token, &refresh_token);

        let tracks = vec![
            "spotify:track:4iV5W9uYEdYUVa79Axb7Rh".to_string(),
            "spotify:episode:512ojhOuo1ktJprKbVcKyQ".to_string(),
        ];

        let tracks = tracks.into_iter().map(|uri| (uri, None)).collect();

        let request = RemoveItemsRequest {
            playlist_id: "3PJeEBUHSVRa1rJ6KOP30H".to_string(),
            tracks,
            ..Default::default()
        };

        let _ = client.remove_items(request).await.unwrap();
    }

    #[tokio::test]
    async fn reorder() {
        dotenv::dotenv().ok();

        let access_token = std::env::var("ACCESS_TOKEN").unwrap();
        let refresh_token = std::env::var("REFRESH_TOKEN").unwrap();

        let mut client = PlaylistClient::new(&access_token, &refresh_token);

        let request = ReorderRequest {
            playlist_id: "3PJeEBUHSVRa1rJ6KOP30H".to_string(),
            range_start: 0,
            insert_before: 2,
            ..Default::default()
        };

        let _ = client.reorder(request).await.unwrap();
    }

    #[tokio::test]
    async fn replace() {
        dotenv::dotenv().ok();

        let access_token = std::env::var("ACCESS_TOKEN").unwrap();
        let refresh_token = std::env::var("REFRESH_TOKEN").unwrap();

        let mut client = PlaylistClient::new(&access_token, &refresh_token);

        let request = ReplaceRequest {
            playlist_id: "3PJeEBUHSVRa1rJ6KOP30H".to_string(),
            uris: vec![
                "spotify:track:4iV5W9uYEdYUVa79Axb7Rh".to_string(),
                "spotify:track:1301WleyT98MSxVHPZCA6M".to_string(),
                "spotify:episode:512ojhOuo1ktJprKbVcKyQ".to_string(),
            ],
        };

        let _ = client.replace(request).await.unwrap();
    }
}
