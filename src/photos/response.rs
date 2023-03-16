use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct UploadPhotoResponse {
    pub upload_url: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UploadedPhotoResponse {
    pub server: i32,
    pub photo: String,
    // user or community id
    pub mid: i32,
    pub hash: String,
    pub message_code: i32,
    pub profile_aid: Option<i32>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SavedOwnerPhotoResponse {
    pub photo_hash: String,
    pub photo_src: String,
    pub photo_src_big: String,
    pub photo_src_small: String
}
