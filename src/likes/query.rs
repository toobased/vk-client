use serde::Serialize;

#[derive(Debug, Serialize, Default)]
pub struct IsLikedQuery {
    pub user_id: Option<String>,
    #[serde(rename = "type")]
    pub media_type: String,
    pub owner_id: String,
    pub item_id: String
}

#[derive(Debug, Serialize, Default)]
pub struct AddLikeQuery {
    #[serde(rename = "type")]
    pub media_type: String,
    pub owner_id: String,
    pub item_id: String,
    pub access_key: Option<String>, // key for private objects
    pub action: Option<String>
}
