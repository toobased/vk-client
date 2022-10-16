use std::{collections::HashMap, fmt::Debug};

use crate::oauth;
use reqwest::{Url, Response};
use serde::{Serialize, de::DeserializeOwned};

use self::response::{BaseVkResponse, VkError};

pub mod response;

#[cfg(test)]
pub mod tests;

#[derive(Debug)]
pub struct VkAppConfig {
    pub app_id: u32,
    pub secret_key: String,
    pub service_key: String
}
impl VkAppConfig {
    pub fn init () -> Self {
        // TODO improve
        let app_id: u32 = std::env::var("vk_app_id").expect("no vk_app_id env var")
            .parse().expect("cant parse vk_app_id env to number");
        let secret_key = std::env::var("vk_app_secret_key").expect("no vk_app_secret_key env var");
        let service_key = std::env::var("vk_app_service_key").expect("no vk_app_service_key env var");
        let config = VkAppConfig { app_id, secret_key, service_key };
        config
    }
}

impl Default for VkAppConfig {
    fn default () -> Self { Self::init() }
}

#[derive(Debug)]
pub struct VkApiConfig {
    pub base_url: String,
    pub version: String
}
impl Default for VkApiConfig {
    fn default() -> Self {
        Self {
            base_url: "https://api.vk.com/method/".to_string(),
            version: "5.131".to_string()
        }
    }
}

#[derive(Debug, Default)]
pub struct VkClient {
    access_token: String,
    pub app_config: VkAppConfig,
    pub api_config: VkApiConfig
}

impl VkClient {

    pub fn get_app_token_oauth_url (&self) -> String {
        let url = oauth::web::get_token_url(
            oauth::web::WebOAuthUrlParams {app_id: self.app_config.app_id, ..Default::default()}
        );
        url.to_string()
    }

    pub fn init (access_token: &str) -> Self {
        Self {
            access_token: access_token.to_string(),
            ..Default::default()
        }
    }

    pub fn init_test () -> Self {
        let token = std::env::var("vk_test_access_token").unwrap();
        Self::init(&token)
    }

    pub async fn base_get<P, T>(&self, method: &str, params: Option<P>) -> BaseVkResponse<T>
        where P: Serialize,
        T: DeserializeOwned
    {
        let base_url = &self.api_config.base_url;
        let api_v = &self.api_config.version;

        let mut url = Url::parse(&base_url).unwrap();
        url.path_segments_mut().unwrap().push(method);
        url.query_pairs_mut()
            .append_pair("v", &api_v)
            .append_pair("access_token", &self.access_token);

        if let Some(p) = params {
            let params_raw = serde_json::to_value::<P>(p).unwrap();
            let params_map = serde_json::from_value::<HashMap<String, Option<String>>>(params_raw).unwrap();
            for (k, v) in params_map.iter() {
                if let Some(value) = v {
                    url.query_pairs_mut().append_pair(&k, &value);
                }
            }
        }

        // TODO
        let resp: Response = reqwest::get(url).await.unwrap();
        let response_log = format!("{:#?}", resp);
        let body = resp.bytes().await.unwrap();
        // TODO
        match serde_json::from_slice::<BaseVkResponse<T>>(&body) {
            Ok(d) => d,
            Err(e) => {
                let log = format!("error: {} body: {:#?} response log: {}", e, body, response_log);
                BaseVkResponse::Error(VkError::parse_data(None, log))
            }
        }
    }
}
