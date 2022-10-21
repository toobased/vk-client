use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Attachment {
    pub photo: Option<VkPhoto>
}

#[derive(Debug, Deserialize)]
pub struct VkPhoto { pub id: i32 }
