use crate::client::{VkClient, response::BaseVkResponse};

pub mod methods;
pub mod query;
pub mod response;

#[cfg(test)]
pub mod tests;

pub async fn get_profile_info (client: &VkClient)
    -> BaseVkResponse<response::GetProfileInfoResponse> {
    client.base_get::<(), response::GetProfileInfoResponse>(
        methods::GET_PROFILE_INFO,
        None
    ).await
}
