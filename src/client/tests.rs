use log::info;

use super::VkClient;

#[test]
fn test_init_client() {
    env_logger::try_init().ok();
    let client = VkClient::init_test();
    info!("client is {:#?}", client);
}

#[test]
fn client_base_functionality () {
    env_logger::try_init().ok();
    let client = VkClient::init_test();
    let web_oauth_url = client.get_app_token_oauth_url();
    info!("web_oauth_url: {}", web_oauth_url)
}
