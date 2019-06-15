use crate::object::Track;
use crate::CountryCode;
use futures::{stream, Future, Stream};
use serde_derive::{Deserialize, Serialize};
use tokio;

#[derive(Deserialize, Serialize, Clone, Debug, Default)]
pub struct Tracks {
    tracks: Vec<Track>,
}

pub fn get_tracks(
    track_ids: &mut Vec<String>,
    market: Option<CountryCode>,
    access_token: String,
) -> Option<Vec<Track>> {
    let ids_list: Vec<Vec<String>> = if track_ids.len() > 50 {
        let mut other_ids = Vec::new();
        while track_ids.len() > 50 {
            let drained: Vec<_> = track_ids.drain(..50).collect();
            other_ids.push(drained);
        }
        other_ids.push(track_ids.to_vec());

        other_ids
    } else {
        vec![track_ids.to_vec()]
    };
    let size = ids_list.len();
    let market = market.map_or("from_token".to_string(), |v| v.alpha2().to_string());
    let client = reqwest::r#async::Client::new();
    let mut rt = tokio::runtime::Runtime::new().unwrap();

    let buffer = stream::iter_ok(ids_list)
        .map(move |ids| {
            let params = [("ids", &ids.join(",")), ("market", &market)];

            client
                .get("https://api.spotify.com/v1/tracks")
                .query(&params)
                .bearer_auth(&access_token)
                .send()
                .and_then(move |mut res| res.json::<Tracks>())
                .then(move |tracks| Ok::<_, std::io::Error>(tracks.unwrap()))
        })
        .buffer_unordered(size);

    match rt.block_on(buffer.collect()) {
        Ok(x) => {
            let mut tracks = Vec::new();
            x.into_iter().for_each(|mut v| tracks.append(&mut v.tracks));

            Some(tracks)
        }
        Err(_) => None,
    }
}
