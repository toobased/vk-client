use serde::Deserialize;

#[derive(Deserialize, Debug)]
// #[serde(untagged)]
#[serde(rename_all = "snake_case")]
pub enum BaseVkResponse<T> {
    Response(T),
    Error(VkError)
}

#[derive(Deserialize, Debug)]
pub struct VkError {
    pub error_code: u32,
    pub error_msg: String,
    pub log: Option<String>
}

impl VkError {
    pub fn parse_data(msg: Option<String>, log: String) -> Self {
        Self {
            error_code: 0,
            error_msg: msg.unwrap_or("Error on parse response data".to_string()),
            log: Some(log)
        }
    }
}
