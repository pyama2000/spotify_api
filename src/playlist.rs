use serde::Deserialize;

#[derive(Clone, Debug, Default, Deserialize)]
pub struct SimplePlaylist {
    pub collaborative: bool,
    pub description: Option<String>,
    // pub external_urls: ExternalURL,
    pub href: String,
    pub id: String,
    // pub images: Vec<Image>,
    pub name: String,
    // pub owner: User,
    pub public: Option<bool>,
    pub snapshot_id: String,
    #[serde(rename = "type")]
    pub object_type: String,
    pub uri: String,
}
