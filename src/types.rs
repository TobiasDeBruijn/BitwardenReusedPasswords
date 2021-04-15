use serde::Deserialize;

#[derive(Deserialize)]
pub struct ExportFile {
    pub encrypted:  bool,
    pub folders:    Vec<String>,
    pub items:      Vec<ExportItem>
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExportItem {
    pub id:                 String,
    pub organization_id:    Option<String>,
    pub folder_id:          Option<String>,

    #[serde(rename(deserialize = "type"))]
    pub item_type:          i64,

    pub name:               String,
    pub notes:              Option<String>,
    pub login:              LoginItem,
    pub collection_ids:     Option<Vec<String>>
}

#[derive(Deserialize)]
pub struct LoginItem {
    pub uris:       Option<Vec<LoginUri>>,
    pub username:   Option<String>,
    pub password:   Option<String>,
    pub totp:       Option<bool>
}

#[derive(Deserialize)]
pub struct LoginUri {
    #[serde(rename(deserialize = "match"))]
    pub uri_match:      Option<i64>,

    pub uri:            String
}