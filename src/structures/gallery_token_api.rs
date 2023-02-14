use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct GalleryToken {
    pub gid: u64,
    pub token: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct GalleryTokenList {
    #[serde(alias = r#"tokenlist"#)]
    pub token_vec_opt: Option<Vec<GalleryToken>>,
    #[serde(alias = r#"error"#)]
    pub error_opt: Option<String>,
}
