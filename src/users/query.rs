use serde::Serialize;

#[derive(Debug, Serialize, Default)]
pub struct GetUserParams {
    pub user_ids: Option<String>,
    pub fields: Option<String>
}
