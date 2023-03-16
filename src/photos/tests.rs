use std::fs;

use crate::client::VkClient;
use log::info;

pub static USER_ID: &str = "721548177";
// pub static GROUP_ID: &str = "-211982694";
// pub static POST_ID: &str = "1318";
//
fn get_test_photo () -> bytes::Bytes {
    let path = "/home/rin/img/universe.webp";
    let b = fs::read(path).unwrap();
    b.into()
}

// #[tokio::test]
async fn test_fetch_user_photos() {
    env_logger::try_init().ok();
    let client = VkClient::init_test();

    let query = crate::photos::query::GetPhotosQuery {
        owner_id: USER_ID.to_string(),
        ..Default::default()
    };
    let res = crate::photos::get(&client, query)
        .await.unwrap();
    info!("get user photos: {} {:#?}", res.count, res.items.get(0))
}

// #[tokio::test]
async fn test_fetch_user_albums () {
    env_logger::try_init().ok();
    let client = VkClient::init_test();

    let query = crate::photos::query::GetAlbumsQuery {
        owner_id: USER_ID.to_string(),
        ..Default::default()
    };
    let res = crate::photos::get_albums(&client, query)
        .await.unwrap();
    info!("get user albums: {:#?} {}", res.items, res.count)
}

// #[tokio::test]
async fn test_get_owner_photo_upload_server () {
    env_logger::try_init().ok();
    let client = VkClient::init_test();

    let query = crate::photos::query::GetOwnerPhotoUploadServerQuery { ..Default::default() };
    let res = crate::photos::get_owner_photo_upload_server(&client, query)
        .await.unwrap();
    info!("upload url is {}", res.upload_url)
}

// #[tokio::test]
async fn test_upload_owner_photo () {
    env_logger::try_init().ok();
    let client = VkClient::init_test();

    let query = crate::photos::query::GetOwnerPhotoUploadServerQuery { ..Default::default() };
    let upload_server = crate::photos::get_owner_photo_upload_server(&client, query)
        .await.unwrap();
    info!("upload url is {}", upload_server.upload_url);
    let uploaded_res = crate::photos::upload_photo(&client, upload_server, get_test_photo())
        .await.unwrap();
    info!("uploaded res is {:?}", uploaded_res);
}

#[tokio::test]
async fn test_save_owner_photo () {
    env_logger::try_init().ok();
    let client = VkClient::init_test();

    let query = crate::photos::query::GetOwnerPhotoUploadServerQuery { ..Default::default() };
    let upload_server = crate::photos::get_owner_photo_upload_server(&client, query)
        .await.unwrap();
    info!("upload url is {}", upload_server.upload_url);
    let uploaded_res = crate::photos::upload_photo(&client, upload_server, get_test_photo())
        .await.unwrap();
    info!("uploaded res is {:?}", uploaded_res);

    let save_query = crate::photos::query::SaveOwnerPhotoQuery {
        hash: uploaded_res.hash,
        server: uploaded_res.server.to_string(),
        photo: uploaded_res.photo
    };

    let saved_res = crate::photos::save_owner_photo(&client, save_query)
        .await.unwrap();
    info!("saved res is {:?}", saved_res)
}
