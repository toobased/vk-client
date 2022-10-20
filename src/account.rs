use crate::client::{VkClient, response::VkError};

pub mod methods;
pub mod query;
pub mod response;

#[cfg(test)]
pub mod tests;

pub async fn get_profile_info (client: &VkClient)
    -> Result<response::GetProfileInfoResponse, VkError> {
    client.base_get::<(), response::GetProfileInfoResponse>(
        methods::GET_PROFILE_INFO,
        None
    ).await
}
