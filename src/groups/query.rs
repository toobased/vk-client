use serde::Serialize;

#[derive(Debug, Serialize, Default)]
pub struct GetByIdQuery {
    pub group_id: String,
    pub group_ids: Option<String>,
    pub fields: Option<String> // TODO
}
