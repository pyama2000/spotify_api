# Spotify\_api

A Rust library for [Spotify Web API](https://developer.spotify.com/documentation/web-api/).

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
spotify_api = { git = "https://github.com/pyama2000/spotify_api" }
```

## Example

```rust
use spotify_api;
use spotify_api::authentication::*;
use spotify_api::user::UserClient as Client;

fn main() {
    let scopes = vec![
        Scope::UserReadPrivate,
        Scope::UserReadBirthdate,
        Scope::UserReadEmail,
    ];

    let mut oauth_client = SpotifyOAuth::new();
    oauth_client.set_scopes(&scopes);
    let url = oauth_client.generate_auth_url().unwrap();

    // Access to `url` and sign in to you account

    let tokens = request_tokens("YOUR CODE").unwrap();

    let mut client = Client::new(tokens.access_token, tokens.refresh_token.unwrap());
    let me = client.get_current_user();
    println!("{:?}", me);
}
```
