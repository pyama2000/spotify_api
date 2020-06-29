use std::error::Error;

use reqwest::{RequestBuilder, Response, StatusCode};

pub mod album;
pub mod authentication;
use authentication::refresh_access_token;

#[derive(Debug)]
pub struct RequestClient {
    client: reqwest::Client,
    access_token: String,
    refresh_token: String,
}

impl RequestClient {
    pub fn new(access_token: &str, refresh_token: &str) -> RequestClient {
        RequestClient {
            client: reqwest::Client::new(),
            access_token: access_token.to_string(),
            refresh_token: refresh_token.to_string(),
        }
    }

    pub async fn send(
        &mut self,
        builder: RequestBuilder,
    ) -> Result<Option<Response>, Box<dyn Error>> {
        loop {
            let response = builder
                .try_clone()
                .unwrap()
                .bearer_auth(&self.access_token)
                .send()
                .await?;

            match response.status() {
                StatusCode::ACCEPTED
                | StatusCode::CREATED
                | StatusCode::NO_CONTENT
                | StatusCode::OK => {
                    return Ok(Some(response));
                }
                StatusCode::UNAUTHORIZED => {
                    self.access_token = refresh_access_token(&self.refresh_token).await?;
                }
                _ => {
                    return Ok(None);
                }
            }
        }
    }
}
