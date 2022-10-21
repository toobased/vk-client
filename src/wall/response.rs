use serde::Deserialize;

use crate::attachments::models::Attachment;

#[derive(Debug, Deserialize)]
pub struct WallPost {
    pub id: i32,
    pub owner_id: i32,
    pub from_id: i32,
    pub created_by: i32,
    pub text: String,
    pub reply_owner_id: Option<u32>,
    pub reply_post_id: Option<u32>,
    pub friends_only: Option<u8>,
    pub likes: super::types::WallPostLike,
    pub reposts: super::types::WallPostRepost,
    pub views: super::types::WallPostView,
    pub post_type: String,
    pub attachments: Vec<Attachment>,
    pub can_pin: Option<u8>,
    pub can_delete: Option<u8>,
    pub can_edit: Option<u8>,
    pub is_pinned: Option<u8>,
    pub is_favorite: bool,
    pub postponed_id: Option<i32>
}

