use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct User {
    pub id: u32,
    pub first_name: String,
    pub last_name: String,
    pub deactivated: Option<String>,
    pub is_closed: bool,
    pub can_access_closed: bool
}
