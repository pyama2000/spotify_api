extern crate spotify_api;
use spotify_api::album::*;

#[tokio::test]
async fn get_album() {
    dotenv::dotenv().ok();

    let access_token = std::env::var("ACCESS_TOKEN").unwrap();
    let refresh_token = std::env::var("REFRESH_TOKEN").unwrap();

    let mut client = AlbumClient::new(&access_token, &refresh_token);
    let request = GetAlbumRequest {
        id: "0sNOF9WDwhWunNAHPD3Baj".to_string(),
        ..Default::default()
    };
    let album = client.get_album(request).await;

    assert_eq!("She\'s So Unusual".to_string(), album.unwrap().name);
}

#[tokio::test]
async fn get_albums() {
    dotenv::dotenv().ok();

    let access_token = std::env::var("ACCESS_TOKEN").unwrap();
    let refresh_token = std::env::var("REFRESH_TOKEN").unwrap();

    let mut client = AlbumClient::new(&access_token, &refresh_token);
    let ids = vec!["41MnTivkwTO3UUJ8DrqEJJ", "6JWc4iAiJ9FjyK0B59ABb4", "6UXCm6bOO4gFlDQZV5yL37"]
        .into_iter()
        .map(|s| s.to_string())
        .collect();
    let request = GetAlbumListRequest {
        ids,
        ..Default::default()
    };
    let response = client.get_albums(request).await.unwrap();

    assert_eq!(3, response.albums.len());
}

#[tokio::test]
async fn get_tracks() {
    dotenv::dotenv().ok();

    let access_token = std::env::var("ACCESS_TOKEN").unwrap();
    let refresh_token = std::env::var("REFRESH_TOKEN").unwrap();

    let mut client = AlbumClient::new(&access_token, &refresh_token);
    let request = GetTrackListRequest {
        id: "6akEvsycLGftJxYudPjmqK".to_string(),
        limit: Some(2),
        ..Default::default()
    };
    let tracks = client.get_tracks(request).await.unwrap().get_items();

    assert_eq!(2, tracks.len());
}
