use dotenv::dotenv;
use failure::Error;
use rand::{self, distributions::Alphanumeric, Rng};
use reqwest;
use serde_derive::Deserialize;
use std::env;
use std::fmt;

#[derive(Default)]
struct ClientCredential {
    client_id: String,
    client_secret: String,
    redirect_uri: String,
}

impl ClientCredential {
    fn new() -> Self {
        dotenv().ok();

        let client_id = env::var("CLIENT_ID").expect("CLIENT_ID must be set");
        let client_secret = env::var("CLIENT_SECRET").expect("CLIENT_SECRET must be set");
        let redirect_uri = env::var("REDIRECT_URI").expect("REDIRECT_URI must be set");

        ClientCredential {
            client_id,
            client_secret,
            redirect_uri,
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum Scope {
    UserReadPrivate,
    UserReadBirthdate,
    UserReadEmail,
    Streaming,
    AppRemoteControl,
    UserTopRead,
    UserReadRecentlyPlayed,
    UserLibraryRead,
    UserLibraryModify,
    PlaylistReadCollaborative,
    PlaylistReadPrivate,
    PlaylistModifyPublic,
    PlaylistModifyPrivate,
    UserReadCurrentlyPlaying,
    UserReadPlaybackState,
    UserModifyPlaybackState,
    UserFollowRead,
    UserFollowModify,
}

impl fmt::Display for Scope {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Scope::UserReadPrivate => write!(f, "user-read-private"),
            Scope::UserReadBirthdate => write!(f, "user-read-birthdate"),
            Scope::UserReadEmail => write!(f, "user-read-email"),
            Scope::Streaming => write!(f, "streaming"),
            Scope::AppRemoteControl => write!(f, "app-remote-control"),
            Scope::UserTopRead => write!(f, "user-top-read"),
            Scope::UserReadRecentlyPlayed => write!(f, "user-read-recently-played"),
            Scope::UserLibraryRead => write!(f, "user-library-read"),
            Scope::UserLibraryModify => write!(f, "user-library-modify"),
            Scope::PlaylistReadCollaborative => write!(f, "playlist-read-collaborative"),
            Scope::PlaylistReadPrivate => write!(f, "playlist-read-private"),
            Scope::PlaylistModifyPublic => write!(f, "playlist-modify-public"),
            Scope::PlaylistModifyPrivate => write!(f, "playlist-modify-private"),
            Scope::UserReadCurrentlyPlaying => write!(f, "user-read-currently-playing"),
            Scope::UserReadPlaybackState => write!(f, "user-read-playback-state"),
            Scope::UserModifyPlaybackState => write!(f, "user-modify-playback-state"),
            Scope::UserFollowRead => write!(f, "user-follow-read"),
            Scope::UserFollowModify => write!(f, "user-follow-modify"),
        }
    }
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct SpotifyOAuth {
    client_id: String,
    redirect_uri: String,
    state: String,
    scopes: Vec<Scope>,
    show_dialog: bool,
}

impl SpotifyOAuth {
    pub fn new() -> Self {
        let credential_info = ClientCredential::new();

        SpotifyOAuth {
            client_id: credential_info.client_id,
            redirect_uri: credential_info.redirect_uri,
            state: generate_random_string(12),
            scopes: Vec::new(),
            show_dialog: false,
        }
    }

    pub fn set_scopes(&mut self, scopes: &[Scope]) -> &mut Self {
        self.scopes = scopes.to_vec();
        self
    }

    pub fn generate_random_state(&mut self, length: usize) -> &mut Self {
        self.state = generate_random_string(length);
        self
    }

    pub fn generate_auth_url(&self) -> Result<String, Error> {
        let scopes = self
            .scopes
            .iter()
            .map(std::string::ToString::to_string)
            .collect::<Vec<String>>()
            .join(" ");
        let query = [
            ("client_id", &self.client_id),
            ("response_type", &"code".to_string()),
            ("redirect_uri", &self.redirect_uri),
            ("state", &self.state),
            ("scope", &scopes),
            ("show_dialog", &self.show_dialog.to_string()),
        ];

        let url =
            reqwest::Url::parse_with_params("https://accounts.spotify.com/authorize", &query)?;

        Ok(url.to_string())
    }
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Deserialize)]
pub struct Token {
    pub access_token: String,
    pub token_type: String,
    pub scope: String,
    pub expires_in: isize,
    pub refresh_token: Option<String>,
}

fn generate_random_string(length: usize) -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(length)
        .collect()
}

pub fn refresh_access_token(refresh_token: &str) -> Result<String, Error> {
    let credential_info = ClientCredential::new();
    let query = [
        ("grant_type", "refresh_token"),
        ("refresh_token", refresh_token),
    ];
    let mut response = reqwest::Client::new()
        .post("https://accounts.spotify.com/api/token")
        .basic_auth(
            credential_info.client_id,
            Some(credential_info.client_secret),
        )
        .form(&query)
        .send()?;
    let token: Token = response.json()?;

    Ok(token.access_token)
}

pub fn request_tokens(code: &str) -> Result<Token, Error> {
    let credential_info = ClientCredential::new();
    let query = [
        ("grant_type", "authorization_code"),
        ("code", code),
        ("redirect_uri", &credential_info.redirect_uri),
    ];
    let mut response = reqwest::Client::new()
        .post("https://accounts.spotify.com/api/token")
        .basic_auth(
            credential_info.client_id,
            Some(credential_info.client_secret),
        )
        .form(&query)
        .send()?;

    Ok(response.json()?)
}