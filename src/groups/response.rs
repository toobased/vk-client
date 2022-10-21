use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct VkGroup {
    pub id: i32,
    pub name: String,
    pub screen_name: String,
    // 0 - open, 1 - closed, 2 - private
    pub is_closed: u8,
    // returns, if community is deleted or banned
    // deleted | banned 
    pub deactivated: Option<String>,
    // is current user admin
    // 1 | 0
    pub is_admin: Option<u8>,
    // 1 - moderator, 2 - editor, 3 - admin
    pub admin_level: Option<u8>,
    // 1 | 0
    pub is_member: Option<u8>,
    // group | page | event TODO enum
    #[serde(rename = "type")]
    pub group_type: String,
    pub photo_50: String,
    pub photo_100: String,
    pub photo_200: String
}
