use serde::Serialize;

#[derive(Debug, Serialize, Default)]
pub struct GetQuery {
    pub owner_id: String,
    pub doamin: String,
    pub offset: String,
    pub count: String,
    pub filter: String,
    pub extend: String,
    pub fields: String
}

#[derive(Debug, Serialize, Default)]
pub struct GetByIdQuery {
    pub posts: String,
    // 1 | 0
    pub extended: String,
    pub copy_history_depth: Option<String>,
    pub fields: Option<String> // TODO
}
