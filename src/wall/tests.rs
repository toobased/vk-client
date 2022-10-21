use crate::client::VkClient;
use log::info;

pub static GROUP_ID: &str = "-211982694";
pub static POST_ID: &str = "1318";

#[tokio::test]
async fn test_get_by_id() {
    env_logger::try_init().ok();
    let client = VkClient::init_test();
    let id_info = format!("{}_{}", GROUP_ID, POST_ID);
    info!("getting post: {}", id_info);
    let query = crate::wall::query::GetByIdQuery {
        posts: id_info,
        ..Default::default()
    };
    let mut result = crate::wall::get_by_id(&client, query)
        .await.unwrap();
    let post = result.get_mut(0).unwrap();
    info!("get wall post by id: {:#?}", post);
}

#[tokio::test]
async fn get_wall_posts() {
    env_logger::try_init().ok();
    let client = VkClient::init_test();
    let query = crate::wall::query::GetQuery {
        owner_id: GROUP_ID.to_string(),
        ..Default::default()
    };
    let result = crate::wall::get(&client, query)
        .await.unwrap();
    info!("get wall posts result: {:#?}", result);
}
