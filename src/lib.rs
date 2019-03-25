pub mod album;
pub mod artist;
pub mod authentication;
pub mod browse;
pub mod follow;
pub mod library;
pub mod object;
pub mod personalization;
pub mod player;
pub mod playlist;
pub mod search;
pub mod track;
pub mod user;

use crate::authentication::refresh_access_token;
pub use chrono::{DateTime, Utc};
use failure::Error;
pub use isocountry::CountryCode;
use reqwest::{RequestBuilder, Response, StatusCode};
use serde::de::DeserializeOwned;
use serde_json;

fn get_value<T: DeserializeOwned>(json: &str, key: &str) -> Result<T, Error> {
    //dbg!(&json);
    let mut value: serde_json::Value = serde_json::from_str(json)?;
    Ok(serde_json::from_value(value[key].take())?)
}

fn get_values<T: DeserializeOwned>(json: &str, key: &str) -> Result<Vec<T>, Error> {
    let mut value: serde_json::Value = serde_json::from_str(json)?;
    Ok(serde_json::from_value(value[key].take())?)
}

fn generate_params(limit: Option<u32>, offset: Option<u32>) -> Vec<(&'static str, String)> {
    let limit = limit.filter(|&x| x <= 50).unwrap_or(20);
    let offset = offset.unwrap_or(0);

    vec![("limit", limit.to_string()), ("offset", offset.to_string())]
}

trait Client {
    fn get_access_token(&self) -> String;
    fn get_refresh_token(&self) -> String;
    fn set_access_token(&mut self, access_token: &str) -> &mut dyn Client;

    fn send(&mut self, request_builder: RequestBuilder) -> Option<Response> {
        let response = request_builder
            .try_clone()?
            .bearer_auth(&self.get_access_token())
            .send()
            .unwrap();

        match response.status() {
            StatusCode::OK
            | StatusCode::CREATED
            | StatusCode::ACCEPTED
            | StatusCode::NO_CONTENT => Some(response),
            StatusCode::UNAUTHORIZED => {
                let access_token = refresh_access_token(&self.get_refresh_token()).unwrap();
                self.set_access_token(&access_token);
                self.send(request_builder)
            }
            StatusCode::TOO_MANY_REQUESTS => {
                dbg!(&response);
                None
            }
            StatusCode::GATEWAY_TIMEOUT => {
                println!("504 Gateway Timeout");
                self.send(request_builder)
            }
            _ => {
                dbg!(&request_builder);
                dbg!(&response);
                None
            }
        }
    }
}
