extern crate spotify_api;

#[cfg(test)]
mod personalization {
    use spotify_api::personalization::*;

    #[tokio::test]
    async fn get_top() {
        dotenv::dotenv().ok();

        let access_token = std::env::var("ACCESS_TOKEN").unwrap();
        let refresh_token = std::env::var("REFRESH_TOKEN").unwrap();

        let mut client = PersonalizationClient::new(&access_token, &refresh_token);

        let artists_request = GetTopRequest {
            limit: Some(2),
            ..Default::default()
        };
        let _ = client.get_top_artists(artists_request).await.unwrap();

        let tracks_request = GetTopRequest {
            limit: Some(2),
            ..Default::default()
        };
        let _ = client.get_top_tracks(tracks_request).await.unwrap();
    }
}
