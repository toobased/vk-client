use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct IsLikedResponse {
    pub liked: u32,
    pub copied: u32
}

#[derive(Debug, Deserialize)]
pub struct AddLikeResponse { pub likes: u32, }
