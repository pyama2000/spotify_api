extern crate spotify_api;
use spotify_api::authentication::*;

#[test]
#[ignore]
fn test_gen_auth_url() {
    let scopes = vec![
        Scope::UserReadPrivate,
        Scope::UserReadEmail,
        Scope::Streaming,
        Scope::AppRemoteControl,
        Scope::UserTopRead,
        Scope::UserReadRecentlyPlayed,
        Scope::UserLibraryRead,
        Scope::UserLibraryModify,
        Scope::PlaylistReadCollaborative,
        Scope::PlaylistReadPrivate,
        Scope::PlaylistModifyPublic,
        Scope::PlaylistModifyPrivate,
        Scope::UserReadCurrentlyPlaying,
        Scope::UserReadPlaybackState,
        Scope::UserModifyPlaybackState,
        Scope::UserFollowRead,
        Scope::UserFollowModify,
    ];

    let mut oauth = SpotifyOAuth::new();
    oauth.set_scopes(&scopes);

    let url = oauth.generate_auth_url().unwrap();
    dbg!(&url);
}

#[tokio::test]
#[ignore]
async fn get_tokens() {
    let code = "AQDdTBFi6wOM2QwqNxutOSe84nqu7XuRunRpk2g6Z7he-3YyaSfkCAhj-fULcmyNZG3qgszsKC0oCE-5OI4Q9CWW2lqcvzfTm2HFSwSnGGA_HuT3i7xYLj5cIowq9jA6FE0Y76BdMNIkmo6A4jNShluJ5m5-oZHPPZGy9P2p9r4AGfrDg9l5BWGAz7L7QAHQWTPLEjvB81VbZwv89WX-vCflbygZO5u3K6P_Vzp5VivGKBTYIhCW3AEGTscB-_i2JvpeoMba9icS17QSEBMigxZOs9nGtegcyjkMQ2PE5MRALl04ukZddxtaBWwiVJPKBdJR8kCcI0L2OXMcvHL6O28wxmm19QefmS626kZssilx4OBLtkbmcoc3TjDGZNpst-kGA9dvJrdflhqYHxXGDylNukF94H2W7RRKcBvwYXd1aMumEVRmyFOeYI5bmmJFaForD72_iYwBRClF6KBKBIqzRX8LLpIcxs4PR7-9iQDozOr9gx0oPznbHPnf4M6NQtXcVq9wWa6D3pqjkaEtiYPdhHgNYx0luAOTD-JOkFYMIIPAeOvV5yQH79b9_Y6UXGvGD4AVNNUKxA7fScg51_3TRKPIzmMTLx4QolXIwK2cgdlWfxcvSerb2_oA8dzeEEcQi-AxRA3yl1C9";

    let token = request_tokens(code).await.unwrap();
    dbg!(&token);
}

#[tokio::test]
#[ignore]
async fn refresh_token() {
    dotenv::dotenv().ok();

    let refresh_token = std::env::var("REFRESH_TOKEN").unwrap();
    let response = refresh_access_token(&refresh_token).await.unwrap();

    dbg!(&response);
}
