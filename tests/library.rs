extern crate spotify_api;

#[cfg(test)]
mod library {
    use spotify_api::library::*;

    #[tokio::test]
    async fn is_saved() {
        dotenv::dotenv().ok();

        let access_token = std::env::var("ACCESS_TOKEN").unwrap();
        let refresh_token = std::env::var("REFRESH_TOKEN").unwrap();

        let mut client = LibraryClient::new(&access_token, &refresh_token);

        let albums_request = CheckSavedRequest {
            ids: vec![
                "0pJJgBzj26qnE1nSQUxaB0".to_string(),
                "5ZAKzV4ZIa5Gt7z29OYHv0".to_string(),
            ],
        };
        let albums_results = client.is_saved_albums(albums_request).await.unwrap();
        albums_results
            .into_iter()
            .for_each(|result| assert!(!result));

        let shows_request = CheckSavedRequest {
            ids: vec!["5AvwZVawapvyhJUIx71pdJ".to_string()],
        };

        let shows_results = client.is_saved_shows(shows_request).await.unwrap();
        shows_results
            .into_iter()
            .for_each(|result| assert!(!result));

        let tracks_request = CheckSavedRequest {
            ids: vec![
                "0udZHhCi7p1YzMlvI4fXoK".to_string(),
                "3SF5puV5eb6bgRSxBeMOk9".to_string(),
            ],
        };

        let tracks_results = client.is_saved_tracks(tracks_request).await.unwrap();
        tracks_results
            .into_iter()
            .for_each(|result| assert!(!result));
    }

    #[tokio::test]
    #[ignore]
    async fn get_saved() {
        dotenv::dotenv().ok();

        let access_token = std::env::var("ACCESS_TOKEN").unwrap();
        let refresh_token = std::env::var("REFRESH_TOKEN").unwrap();

        let mut client = LibraryClient::new(&access_token, &refresh_token);

        let albums_request = GetSavedRequest {
            limit: Some(2),
            ..Default::default()
        };
        let _ = client.get_saved_albums(albums_request).await.unwrap();

        let shows_request = GetSavedRequest {
            limit: Some(2),
            ..Default::default()
        };
        let _ = client.get_saved_shows(shows_request).await.unwrap();

        let tracks_request = GetSavedRequest {
            limit: Some(2),
            ..Default::default()
        };
        let _ = client.get_saved_tracks(tracks_request).await.unwrap();
    }

    #[tokio::test]
    #[ignore]
    async fn save() {
        dotenv::dotenv().ok();

        let access_token = std::env::var("ACCESS_TOKEN").unwrap();
        let refresh_token = std::env::var("REFRESH_TOKEN").unwrap();

        let mut client = LibraryClient::new(&access_token, &refresh_token);

        let albums_request = SaveRequest {
            ids: vec![
                "0pJJgBzj26qnE1nSQUxaB0".to_string(),
                "5ZAKzV4ZIa5Gt7z29OYHv0".to_string(),
            ],
        };
        let _ = client.save_albums(albums_request).await.unwrap();

        let shows_request = SaveRequest {
            ids: vec!["5AvwZVawapvyhJUIx71pdJ".to_string()],
        };
        let _ = client.save_shows(shows_request).await.unwrap();

        let tracks_request = SaveRequest {
            ids: vec![
                "0udZHhCi7p1YzMlvI4fXoK".to_string(),
                "3SF5puV5eb6bgRSxBeMOk9".to_string(),
            ],
        };
        let _ = client.save_tracks(tracks_request).await.unwrap();
    }

    #[tokio::test]
    #[ignore]
    async fn remove() {
        dotenv::dotenv().ok();

        let access_token = std::env::var("ACCESS_TOKEN").unwrap();
        let refresh_token = std::env::var("REFRESH_TOKEN").unwrap();

        let mut client = LibraryClient::new(&access_token, &refresh_token);

        let albums_request = RemoveSavedRequest {
            ids: vec![
                "0pJJgBzj26qnE1nSQUxaB0".to_string(),
                "5ZAKzV4ZIa5Gt7z29OYHv0".to_string(),
            ],
        };
        let _ = client.remove_saved_albums(albums_request).await.unwrap();

        let shows_request = RemoveSavedRequest {
            ids: vec!["5AvwZVawapvyhJUIx71pdJ".to_string()],
        };
        let _ = client.remove_saved_shows(shows_request).await.unwrap();

        let tracks_request = RemoveSavedRequest {
            ids: vec![
                "0udZHhCi7p1YzMlvI4fXoK".to_string(),
                "3SF5puV5eb6bgRSxBeMOk9".to_string(),
            ],
        };
        let _ = client.remove_saved_tracks(tracks_request).await.unwrap();
    }
}
