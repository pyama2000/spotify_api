extern crate spotify_api;

#[cfg(test)]
mod follow {
    use spotify_api::follow::*;

    #[tokio::test]
    async fn is_following() {
        dotenv::dotenv().ok();

        let access_token = std::env::var("ACCESS_TOKEN").unwrap();
        let refresh_token = std::env::var("REFRESH_TOKEN").unwrap();

        let mut client = FollowClient::new(&access_token, &refresh_token);

        let request = CheckFollowRequest {
            ids: vec!["exampleuser01".to_string()],
        };

        let result = client.is_following_user(request).await.unwrap();

        assert!(!result.first().unwrap());

        let request = CheckFollowRequest {
            ids: vec![
                "74ASZWbe4lXaubB36ztrGX".to_string(),
                "08td7MxkoHQkXnWAYD8d6Q".to_string(),
            ],
        };

        let result = client.is_following_artist(request).await.unwrap();
        assert!(!result.first().unwrap());
        assert!(!result.get(1).unwrap());
    }

    #[tokio::test]
    async fn is_users_following_playlist() {
        dotenv::dotenv().ok();

        let access_token = std::env::var("ACCESS_TOKEN").unwrap();
        let refresh_token = std::env::var("REFRESH_TOKEN").unwrap();

        let mut client = FollowClient::new(&access_token, &refresh_token);
        let request = CheckUserFollowPlaylistRequest {
            playlist_id: "2v3iNvBX8Ay1Gt2uXtUKUT".to_string(),
            user_ids: vec!["possan".to_string(), "elogain".to_string()],
        };
        let results = client.is_users_following_playlist(request).await.unwrap();

        assert!(!results.get(0).unwrap());
        assert!(!results.get(1).unwrap());
    }

    #[tokio::test]
    async fn follow() {
        dotenv::dotenv().ok();

        let access_token = std::env::var("ACCESS_TOKEN").unwrap();
        let refresh_token = std::env::var("REFRESH_TOKEN").unwrap();

        let mut client = FollowClient::new(&access_token, &refresh_token);

        let request = FollowRequest {
            ids: vec!["exampleuser01".to_string()],
        };
        let _ = client.follow_users(request).await.unwrap();

        let request = FollowRequest {
            ids: vec![
                "74ASZWbe4lXaubB36ztrGX".to_string(),
                "08td7MxkoHQkXnWAYD8d6Q".to_string(),
            ],
        };
        let _ = client.follow_artists(request).await.unwrap();
    }

    #[tokio::test]
    async fn follow_playlist() {
        dotenv::dotenv().ok();

        let access_token = std::env::var("ACCESS_TOKEN").unwrap();
        let refresh_token = std::env::var("REFRESH_TOKEN").unwrap();

        let mut client = FollowClient::new(&access_token, &refresh_token);

        let request = FollowPlaylistRequest {
            id: "2v3iNvBX8Ay1Gt2uXtUKUT".to_string(),
            ..Default::default()
        };
        let _ = client.follow_playlist(request).await.unwrap();
    }

    #[tokio::test]
    async fn get_followed_artists() {
        dotenv::dotenv().ok();

        let access_token = std::env::var("ACCESS_TOKEN").unwrap();
        let refresh_token = std::env::var("REFRESH_TOKEN").unwrap();

        let mut client = FollowClient::new(&access_token, &refresh_token);
        let request = GetUserFollowedArtistRequest {
            limit: Some(2),
            ..Default::default()
        };
        let artists = client.get_followed_artists(request).await.unwrap().artists;

        assert_eq!(2, artists.get_items().len());
    }

    #[tokio::test]
    async fn unfollow() {
        dotenv::dotenv().ok();

        let access_token = std::env::var("ACCESS_TOKEN").unwrap();
        let refresh_token = std::env::var("REFRESH_TOKEN").unwrap();

        let mut client = FollowClient::new(&access_token, &refresh_token);

        let request = UnfollowRequest {
            ids: vec!["exampleuser01".to_string()],
        };
        let _ = client.unfollow_users(request).await.unwrap();

        let request = UnfollowRequest {
            ids: vec![
                "74ASZWbe4lXaubB36ztrGX".to_string(),
                "08td7MxkoHQkXnWAYD8d6Q".to_string(),
            ],
        };
        let _ = client.unfollow_artists(request).await.unwrap();
    }

    #[tokio::test]
    async fn unfollow_playlist() {
        dotenv::dotenv().ok();

        let access_token = std::env::var("ACCESS_TOKEN").unwrap();
        let refresh_token = std::env::var("REFRESH_TOKEN").unwrap();

        let mut client = FollowClient::new(&access_token, &refresh_token);

        let request = UnfollowPlaylistRequest {
            id: "2v3iNvBX8Ay1Gt2uXtUKUT".to_string(),
            ..Default::default()
        };
        let _ = client.unfollow_playlist(request).await.unwrap();
    }
}
