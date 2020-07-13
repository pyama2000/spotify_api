use std::error::Error;

use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::RequestClient;

async fn get_paging_object<T: DeserializeOwned>(
    url: &str,
    access_token: &str,
    refresh_token: &str,
) -> Result<T, Box<dyn Error>> {
    let mut client = RequestClient::new(access_token, refresh_token);
    let request = reqwest::Client::new().get(url);
    let response = client.send(request).await?.unwrap();

    Ok(response.json().await?)
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PagingObject<T> {
    pub href: String,
    pub items: Vec<T>,
    pub limit: u32,
    pub next: Option<String>,
    pub offset: Option<u32>,
    pub previous: Option<String>,
    pub total: Option<u32>,
}

impl<T: DeserializeOwned + Clone> PagingObject<T> {
    pub async fn get_next(
        &self,
        access_token: &str,
        refresh_token: &str,
    ) -> Result<Option<PagingObject<T>>, Box<dyn Error>> {
        let object = if let Some(url) = &self.next {
            Some(get_paging_object(url, access_token, refresh_token).await?)
        } else {
            None
        };

        Ok(object)
    }

    pub async fn get_previous(
        &self,
        access_token: &str,
        refresh_token: &str,
    ) -> Result<Option<PagingObject<T>>, Box<dyn Error>> {
        let object = if let Some(url) = &self.previous {
            Some(get_paging_object(url, access_token, refresh_token).await?)
        } else {
            None
        };

        Ok(object)
    }

    pub fn get_items(&self) -> Vec<T> {
        self.items.clone()
    }

    pub async fn get_all_items(
        &self,
        access_token: &str,
        refresh_token: &str,
    ) -> Result<Vec<T>, Box<dyn Error>> {
        let mut items: Vec<T> = Vec::new();
        let mut previous = self.get_previous(access_token, refresh_token).await?;

        while let Some(p) = previous {
            let mut prev_items: Vec<T> = p.get_items();
            prev_items.reverse();
            items.append(&mut prev_items);
            previous = p.get_previous(access_token, refresh_token).await?;
        }

        items.reverse();
        let mut current_items: Vec<T> = self.get_items();
        items.append(&mut current_items);

        let mut next = self.get_next(access_token, refresh_token).await?;
        while let Some(n) = next {
            let mut next_items: Vec<T> = n.get_items();
            items.append(&mut next_items);
            next = n.get_next(access_token, refresh_token).await?;
        }

        Ok(items)
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Cursor {
    after: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CursorPagingObject<T> {
    pub href: String,
    pub items: Vec<T>,
    pub limit: u32,
    pub next: Option<String>,
    pub cursors: Cursor,
    pub total: Option<u32>,
}

impl<T: DeserializeOwned + Clone> CursorPagingObject<T> {
    pub async fn get_next(
        &self,
        access_token: &str,
        refresh_token: &str,
    ) -> Result<Option<CursorPagingObject<T>>, Box<dyn Error>> {
        let object = if let Some(url) = &self.next {
            Some(get_paging_object(url, access_token, refresh_token).await?)
        } else {
            None
        };

        Ok(object)
    }

    pub fn get_items(&self) -> Vec<T> {
        self.items.clone()
    }

    pub async fn get_all_items(
        &self,
        access_token: &str,
        refresh_token: &str,
    ) -> Result<Vec<T>, Box<dyn Error>> {
        let mut items = self.get_items();

        let mut next = self.get_next(access_token, refresh_token).await?;
        while let Some(n) = next {
            items.append(&mut n.get_items());
            next = n.get_next(access_token, refresh_token).await?;
        }

        Ok(items)
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Image {
    pub height: Option<u32>,
    pub url: String,
    pub width: Option<u32>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Follower {
    pub href: Option<String>,
    pub total: u32,
}
