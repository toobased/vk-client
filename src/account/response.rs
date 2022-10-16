use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct GetProfileInfoResponse {
    pub id: u32,
    pub first_name: String,
    pub last_name: String,
    pub screen_name: Option<String>
}
