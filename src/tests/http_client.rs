use serde::Deserialize;
use log::info;

#[tokio::test]
async fn test_init_client() {
    env_logger::init();
    let url = "https://jsonplaceholder.typicode.com/users";
    let resp: reqwest::Response = reqwest::get(url).await
        .unwrap();
    let data = resp.json::<Vec<User>>().await
        .unwrap();
    info!("data is {:#?}", data);
}

#[derive(Deserialize, Debug)]
pub struct User {
    pub id: i32,
    #[allow(unused)]
    name: String
}
