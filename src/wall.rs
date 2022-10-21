use crate::{client::{response::VkError, VkClient}, common::ItemsResult};

pub mod methods;
pub mod query;
pub mod response;
pub mod types;

#[cfg(test)]
pub mod tests;

pub async fn get(client: &VkClient, params: query::GetQuery)
    -> Result<ItemsResult<response::WallPost>, VkError> {
    client.base_get::<query::GetQuery, ItemsResult<response::WallPost>>(
        methods::GET,
        Some(params)
    ).await
}

pub async fn get_by_id(client: &VkClient, params: query::GetByIdQuery)
    -> Result<Vec<response::WallPost>, VkError> {
    client.base_get::<query::GetByIdQuery, Vec<response::WallPost>>(
        methods::GET_BY_ID,
        Some(params)
    ).await
}

