use crate::client::VkClient;
use log::info;

#[tokio::test]
async fn test_get_profile_info () {
    env_logger::try_init().ok();
    let client = VkClient::init_test();
    let profile_result = crate::account::get_profile_info(&client).await;
    info!("get profile response is: {:#?}", profile_result);
}
