use std::error::Error;

use serde::Deserialize;

use crate::{object::Image, RequestClient};

#[derive(Clone, Debug, Default, Deserialize)]
pub struct User {
    pub birthdate: Option<String>,
    pub country: Option<String>,
    pub email: Option<String>,
    pub display_name: Option<String>,
    // pub external_urls: ExternalURL,
    // pub followers: Option<Follower>,
    pub href: String,
    pub id: String,
    pub images: Option<Vec<Image>>,
    pub product: Option<String>,
    #[serde(rename = "type")]
    pub object_type: String,
    pub uri: String,
}

#[derive(Clone, Debug, Default)]
pub struct UserClient {
    client: RequestClient,
}

impl UserClient {
    pub fn new(access_token: &str, refresh_token: &str) -> Self {
        UserClient {
            client: RequestClient::new(access_token, refresh_token),
        }
    }

    pub async fn get_current_user(&mut self) -> Result<User, Box<dyn Error>>{
        let builder = reqwest::Client::new().get("https://api.spotify.com/v1/me");

        let response = self.client.send(builder).await?.unwrap();

        Ok(response.json().await?)
    }

    pub async fn get_user(&mut self, id: &str) -> Result<User, Box<dyn Error>> {
        let url = format!("https://api.spotify.com/v1/users/{}", id);
        let builder = reqwest::Client::new().get(&url);

        let response = self.client.send(builder).await?.unwrap();

        Ok(response.json().await?)
    }
}
