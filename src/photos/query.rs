use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ServiceAlbumId { Wall, Profile, Saved }

impl ToString for ServiceAlbumId {
    fn to_string(&self) -> String {
        match self {
            Self::Wall => "wall".to_string(),
            Self::Profile => "profile".to_string(),
            Self::Saved => "saved".to_string(),
        }
    }
}

#[derive(Debug, Serialize)]
pub enum AlbumQuery {
    Id(String),
    Reserved(ServiceAlbumId)
}


impl ToString for AlbumQuery {
    fn to_string(&self) -> String {
        match self {
            Self::Id(v) => v.to_string(),
            Self::Reserved(v) => v.to_string()
        }
    }
}

impl Default for AlbumQuery { fn default() -> Self {
    AlbumQuery::Reserved(ServiceAlbumId::Profile) }
}

#[derive(Debug, Serialize)]
pub struct GetPhotosQuery {
    pub owner_id: String,
    pub album_id: String,
    // pub photo_ids: Option<Vec<String>>,
    // sort type
    // pub rev: bool,
    // additional fields will be returned
    // pub extended: bool,
    pub offset: String,
    pub count: String
}

impl Default for GetPhotosQuery {
    fn default() -> Self {
        Self {
            owner_id: "".to_string(),
            album_id: AlbumQuery::Reserved(ServiceAlbumId::Profile).to_string(),
            offset: "0".to_string(),
            count: "10".to_string()
        }
    }
}

#[derive(Debug, Serialize)]
pub struct GetAlbumsQuery {
    pub owner_id: String,
    pub album_ids: String,
    pub offset: String,
    pub count: String,
    pub need_system: String
}

impl Default for GetAlbumsQuery {
    fn default() -> Self {
        Self {
            owner_id: "".to_string(),
            // album_ids: AlbumQuery::Reserved(ServiceAlbumId::Profile).to_string(),
            album_ids: "".to_string(),
            offset: "0".to_string(),
            count: "10".to_string(),
            need_system: "1".to_string()
        }
    }
}


#[derive(Debug, Serialize, Default)]
pub struct GetOwnerPhotoUploadServerQuery { pub owner_id: Option<String> }

/**
`photo` - `multipart/form-data` format
*/
#[derive(Debug, Serialize, Default)]
pub struct SaveOwnerPhotoQuery {
    pub server: String,
    pub hash: String,
    pub photo: String
}
