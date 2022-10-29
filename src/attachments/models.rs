use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Attachment {
    pub photo: Option<VkPhoto>
}

#[derive(Debug, Deserialize, Default)]
pub struct VkPhotoSize {
    pub url: String,
    pub width: Option<String>,
    pub height: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct VkPhoto {
    pub id: i32,
    // pub sizes: Vec<VkPhotoSize>
}
