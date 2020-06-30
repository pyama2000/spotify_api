use std::error::Error;

use isocountry::CountryCode;
use reqwest::{RequestBuilder, Response, StatusCode};

pub mod album;
pub mod artist;
pub mod authentication;
pub mod object;
pub mod track;
use authentication::refresh_access_token;

#[derive(Clone, Debug, Default)]
pub struct RequestClient {
    client: reqwest::Client,
    access_token: String,
    refresh_token: String,
    offset: Option<u32>,
    limit: Option<u32>,
    market: Option<CountryCode>,
}

impl RequestClient {
    pub fn new(access_token: &str, refresh_token: &str) -> Self {
        RequestClient {
            client: reqwest::Client::new(),
            access_token: access_token.to_string(),
            refresh_token: refresh_token.to_string(),
            offset: None,
            limit: None,
            market: None,
        }
    }

    pub fn set_offset(&mut self, offset: Option<u32>) -> &mut Self {
        self.offset = offset;
        self
    }

    pub fn set_limit(&mut self, limit: Option<u32>) -> &mut Self {
        self.limit = limit;
        self
    }

    pub fn set_market(&mut self, market: Option<CountryCode>) -> &mut Self {
        self.market = market;
        self
    }

    pub async fn send(
        &mut self,
        mut builder: RequestBuilder,
    ) -> Result<Option<Response>, Box<dyn Error>> {
        if let Some(offset) = &self.offset {
            builder = builder.query(&[("offset", offset)]);
        }
        if let Some(limit) = &self.limit {
            builder = builder.query(&[("limit", limit)]);
        }
        if let Some(market) = &self.market {
            builder = builder.query(&[("market", market.alpha2().to_string())]);
        }

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
