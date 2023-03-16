use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Attachment {
    pub photo: Option<VkPhoto>
}

#[derive(Debug, Deserialize, Default, Clone)]
pub struct VkPhotoSize {
    pub url: String,
    pub width: Option<u32>,
    pub height: Option<u32>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct VkPhoto {
    pub id: i32,
    pub sizes: Vec<VkPhotoSize>
}

#[derive(Debug, Deserialize, Clone)]
pub struct VkAlbum {
    pub id: i32,
    pub thumb_id: i32,
    pub owner_id: u32,
    pub title: String,
    pub description: Option<String>,
    pub size: u32,
    // if current user can upload photo to this album (for communities)
    pub can_upload: Option<bool>,
}
