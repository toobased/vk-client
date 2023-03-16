use crate::client::VkClient;
use log::info;

// #[tokio::test]
async fn test_get_user () {
    env_logger::try_init().ok();
    let client = VkClient::init_test();
    let users_query = crate::users::query::GetUserParams {
        user_ids: Some("557036603".to_string()),
        ..Default::default()
    };
    let users_result = crate::users::get(&client, Some(users_query)).await;
    info!("get profile response is: {:#?}", users_result);
}

#[tokio::test]
async fn test_get_user_me () {
    env_logger::try_init().ok();
    let client = VkClient::init_test();
    let users_query = crate::users::query::GetUserParams {
        // user_ids: Some("557036603".to_string()),
        ..Default::default()
    };
    let users_result = crate::users::get(&client, Some(users_query)).await;
    info!("get profile me response is: {:#?}", users_result);
}
