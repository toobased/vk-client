use crate::client::VkClient;
use log::info;

#[tokio::test]
async fn test_is_liked () {
    env_logger::try_init().ok();
    let client = VkClient::init_test();
    let is_liked_query = crate::likes::query::IsLikedQuery {
        item_id: "1250".to_string(),
        owner_id: "-211982694".to_string(),
        media_type: crate::media::POST.to_string(),
        ..Default::default()
    };
    let is_liked_result = crate::likes::is_liked(&client, is_liked_query).await;
    info!("is liked response is: {:#?}", is_liked_result);
}

#[tokio::test]
async fn test_add_like () {
    env_logger::try_init().ok();
    let client = VkClient::init_test();
    let add_like_query = crate::likes::query::AddLikeQuery {
        item_id: "1250".to_string(),
        owner_id: "-211982694".to_string(),
        media_type: crate::media::POST.to_string(),
        ..Default::default()
    };
    let add_like_result = crate::likes::add(&client, add_like_query).await;
    info!("add like response is: {:#?}", add_like_result);
}
