use crate::object::User;
use crate::Client;
use reqwest;

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct UserClient {
    access_token: String,
    refresh_token: String,
}

impl Client for UserClient {
    fn get_access_token(&self) -> String {
        self.access_token.to_string()
    }

    fn get_refresh_token(&self) -> String {
        self.refresh_token.to_string()
    }

    fn set_access_token(&mut self, access_token: &str) -> &mut Client {
        self.access_token = access_token.to_string();
        self
    }
}

impl UserClient {
    pub fn new(access_token: &str, refresh_token: &str) -> Self {
        UserClient {
            access_token: access_token.to_string(),
            refresh_token: refresh_token.to_string(),
        }
    }

    pub fn get_current_user(&mut self) -> User {
        let request = reqwest::Client::new().get("https://api.spotify.com/v1/me");
        let mut response = self.send(request).unwrap();
        response.json().unwrap()
    }

    pub fn get_user(&mut self, id: &str) -> User {
        let url = format!("https://api.spotify.com/v1/users/{}", id);
        let request = reqwest::Client::new().get(&url);
        let mut response = self.send(request).unwrap();
        response.json().unwrap()
    }
}
