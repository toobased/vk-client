use crate::client::VkClient;
use log::info;

pub static GROUP_ID: &str = "211982694";

#[tokio::test]
async fn test_get_by_id() {
    env_logger::try_init().ok();
    let client = VkClient::init_test();
    let query = crate::groups::query::GetByIdQuery {
        group_id: GROUP_ID.to_string(),
        ..Default::default()
    };
    let mut result = crate::groups::get_by_id(&client, query)
        .await.unwrap();
    let group = result.get_mut(0).unwrap();
    info!("get group response is: {:#?}", group);
}
