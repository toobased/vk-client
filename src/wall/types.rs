use serde::Deserialize;

#[derive(Debug, Deserialize, Default, Clone)]
pub struct WallPostLike {
    pub count: u32,
    // 0 | 1
    pub user_likes: u8,
    // 0 | 1
    pub can_like: u8,
    // 0 | 1
    pub can_publish: u8
}

#[derive(Debug, Deserialize, Default, Clone)]
pub struct WallPostRepost {
    pub count: u32,
    // 0 | 1
    pub user_reposted: u8,
}

#[derive(Debug, Deserialize, Default, Clone)]
pub struct WallPostView { pub count: u32, }
