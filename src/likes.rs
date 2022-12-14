use crate::client::{VkClient, response::VkError};

pub mod methods;
pub mod query;
pub mod response;

#[cfg(test)]
pub mod tests;

pub async fn is_liked (client: &VkClient, params: query::IsLikedQuery)
    -> Result<response::IsLikedResponse, VkError> {
    client.base_get::<query::IsLikedQuery, response::IsLikedResponse>(
        methods::IS_LIKED,
        Some(params)
    ).await
}

pub async fn add (client: &VkClient, params: query::AddLikeQuery)
    -> Result<response::AddLikeResponse, VkError> {
    client.base_get::<query::AddLikeQuery, response::AddLikeResponse>(
        methods::ADD,
        Some(params)
    ).await
}
