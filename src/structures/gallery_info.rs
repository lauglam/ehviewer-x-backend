use crate::structures::{GalleryIdentity, Thumb};

#[derive(Debug, PartialEq)]
pub struct GalleryInfo {
    pub identity: GalleryIdentity,
    pub title: String,
    pub thumb: Thumb,
    pub category: u32,
    pub posted: String,
    /// 1. gallery list page:
    ///     Some: Minimal MinimalPlus Compact Extended
    ///     None: Thumbnail
    ///
    /// 2. favorites page:
    ///     Some: Extended
    ///     None: Minimal MinimalPlus Compact Thumbnail
    pub uploader_opt: Option<String>,
    /// 1. Some: Compact Extended
    /// 2. None: Minimal MinimalPlus Thumbnail
    pub simple_tag_vec_opt: Option<Vec<String>>,
    pub rating: f32,
    pub pages: u32,
    pub simple_language_opt: Option<String>,
    pub is_favorited: bool,
    pub favorite_slot_opt: Option<u32>,
    pub favorite_name_opt: Option<String>,
}
