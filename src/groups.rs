use crate::client::{VkClient, response::VkError};

pub mod response;
pub mod query;
pub mod methods;

#[cfg(test)]
pub mod tests;

pub async fn get_by_id(client: &VkClient, params: query::GetByIdQuery)
    -> Result<Vec<response::VkGroup>, VkError> {
    client.base_get::<query::GetByIdQuery, Vec<response::VkGroup>>(
        methods::GET_BY_ID,
        Some(params)
    ).await
}
