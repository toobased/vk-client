use std::fs;

use reqwest::Url;

use crate::{client::{VkClient, response::{VkError, BaseVkResponse}}, common::ItemsResult, attachments::models::{VkPhoto, VkAlbum}};

pub mod methods;
pub mod query;
pub mod response;

#[cfg(test)]
pub mod tests;

pub async fn get (client: &VkClient, params: query::GetPhotosQuery)
    -> Result<ItemsResult<VkPhoto>, VkError> {
    client.base_get::<query::GetPhotosQuery, ItemsResult<VkPhoto>>(
        methods::GET,
        Some(params)
    ).await
}

pub async fn get_albums (client: &VkClient, params: query::GetAlbumsQuery)
    -> Result<ItemsResult<VkAlbum>, VkError> {
    client.base_get::<query::GetAlbumsQuery, ItemsResult<VkAlbum>>(
        methods::GET_ALBUMS,
        Some(params)
    ).await
}

/** */
pub async fn upload_photo (client: &VkClient, resource: response::UploadPhotoResponse, photo: bytes::Bytes)
    -> Result<response::UploadedPhotoResponse, VkError> {
    let req_client = reqwest::Client::new();
    let upload_url = resource.upload_url.replace("\\", "");
    log::info!("cleaned upload url is {}", upload_url);

    log::info!("photo is {:?}", photo.len());

    let url = Url::parse(&upload_url).unwrap();
    // url.query_pairs_mut()
        // .append_pair("v", &client.api_config.version)
        // .append_pair("access_token", &client.access_token);

    let photo_bytes = photo.to_vec();
    let photo_part = reqwest::multipart::Part::bytes(photo_bytes)
        .file_name("universe.webp")
        .mime_str("imaged/webp").unwrap();
    // let photo_part = reqwest::multipart::Part::text("https://i.scdn.co/image/ab67616d0000b273d7859b708f662c3018f213b8");
    let form = reqwest::multipart::Form::new()
        .part("photo", photo_part);

    let resp = req_client
        .post(url)
        .multipart(form)
        .send().await.unwrap();

    // let req = reqwest::Request::new(reqwest::Method::POST, url);
    // let resp = client.execute(req).await.unwrap();
    let data = resp.json::<response::UploadedPhotoResponse>().await.unwrap();
    Ok(data)
}

/** */
pub async fn get_owner_photo_upload_server (client: &VkClient, params: query::GetOwnerPhotoUploadServerQuery)
    -> Result<response::UploadPhotoResponse, VkError> {
    client.base_get::<query::GetOwnerPhotoUploadServerQuery, response::UploadPhotoResponse>(
        methods::GET_OWNER_PHOTO_UPLOAD_SERVER,
        Some(params)
    ).await
}

/** */
pub async fn save_owner_photo (client: &VkClient, params: query::SaveOwnerPhotoQuery)
    -> Result<response::SavedOwnerPhotoResponse, VkError> {
    let link = format!("{}{}", client.api_config.base_url, methods::SAVE_OWNER_PHOTO);
    let req_client = reqwest::Client::new();

    let url = Url::parse(&link).unwrap();

    // let file_part = reqwest::multipart::Part::text(params.photo);
    let form = reqwest::multipart::Form::new()
        .text("access_token", client.access_token.clone())
        .text("v", client.api_config.version.clone())
        .text("server", params.server)
        .text("hash", params.hash)
        .text("photo", params.photo);
    // let req = reqwest::Request::new(reqwest::Method::POST, url);

    let resp = req_client
        .post(url)
        .multipart(form)
        .send().await.unwrap();

    // let resp = req_client.execute(req).await.unwrap();
    match resp.json::<BaseVkResponse<response::SavedOwnerPhotoResponse>>().await {
        Ok(parsed) => {
            match parsed {
                BaseVkResponse::Response(v) => Ok(v),
                BaseVkResponse::Error(e) => Err(e)
            }
        }
        Err(e) => {
            let log = format!("error: {} body: {:#?} response log: {}", "", e, "");
            Err(VkError::parse_data(None, log))
        }
    }
}
