use crate::object::Playlist;
use crate::CountryCode;
use futures::{stream, Future, Stream};
use tokio;

pub fn get_playlists(
    playlist_ids: Vec<String>,
    market: Option<CountryCode>,
    access_token: String,
) -> Option<Vec<Playlist>> {
    let market = market.map_or("from_token".to_string(), |v| v.alpha2().to_string());
    let size = playlist_ids.len();
    let client = reqwest::r#async::Client::new();
    let mut rt = tokio::runtime::Runtime::new().unwrap();

    let buffer = stream::iter_ok(playlist_ids)
        .map(move |id| {
            let url = format!("https://api.spotify.com/v1/playlists/{}", id);

            client
                .get(&url)
                .query(&[("market", &market)])
                .bearer_auth(&access_token)
                .send()
                .and_then(move |mut res| res.json::<Playlist>())
                .then(move |playlist| Ok::<_, std::io::Error>(playlist.unwrap()))
        })
        .buffer_unordered(size);

    rt.block_on(buffer.collect()).ok()
}
