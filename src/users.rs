use crate::client::{VkClient, response::VkError};

pub mod methods;
pub mod query;
pub mod response;

#[cfg(test)]
pub mod tests;

pub async fn get (client: &VkClient, params: Option<query::GetUserParams>)
    -> Result<Vec<response::VkUser>, VkError> {
    let p = params.unwrap_or(query::GetUserParams{..Default::default()});
    client.base_get::<query::GetUserParams, Vec<response::VkUser>>(
        methods::GET,
        Some(p)
    ).await
}
