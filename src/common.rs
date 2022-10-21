use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ItemsResult<T> {
    pub count: u32,
    pub items: Vec<T>
}
