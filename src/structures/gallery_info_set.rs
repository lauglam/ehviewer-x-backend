use crate::structures::Thumb;

#[derive(Debug, PartialEq)]
pub enum GalleryInfoSet {
    Minimal(Vec<GalleryInfoMinimal>),
    MinimalPlus(Vec<GalleryInfoMinimalPlus>),
    Compact(Vec<GalleryInfoCompact>),
    Extended(Vec<GalleryInfoExtended>),
    Thumbnail(Vec<GalleryInfoThumbnail>),
}

#[derive(Debug, PartialEq)]
pub struct GalleryInfoMinimal {
    pub gid: u64,
    pub token: String,
    pub title: String,
    pub thumb: Thumb,
    pub category: u32,
    pub posted: String,
    /// Minimal MinimalPlus Compact Extended
    pub uploader: String,
    pub rating: f32,
    pub pages: u32,
    pub simple_language_opt: Option<String>,
    pub is_favorited: bool,
    pub favorite_slot_opt: Option<u32>,
    pub favorite_name_opt: Option<String>,
}

#[derive(Debug, PartialEq)]
pub struct GalleryInfoMinimalPlus {
    pub gid: u64,
    pub token: String,
    pub title: String,
    pub thumb: Thumb,
    pub category: u32,
    pub posted: String,
    pub uploader: String,
    pub rating: f32,
    pub pages: u32,
    pub simple_language_opt: Option<String>,
    pub is_favorited: bool,
    pub favorite_slot_opt: Option<u32>,
    pub favorite_name_opt: Option<String>,
}

#[derive(Debug, PartialEq)]
pub struct GalleryInfoCompact {
    pub gid: u64,
    pub token: String,
    pub title: String,
    pub thumb: Thumb,
    pub category: u32,
    pub posted: String,
    /// Minimal MinimalPlus Compact Extended
    pub uploader: String,
    pub rating: f32,
    /// Compact Extended
    pub simple_tag_vec: Vec<String>,
    pub pages: u32,
    pub simple_language_opt: Option<String>,
    pub is_favorited: bool,
    pub favorite_slot_opt: Option<u32>,
    pub favorite_name_opt: Option<String>,
}

#[derive(Debug, PartialEq)]
pub struct GalleryInfoExtended {
    pub gid: u64,
    pub token: String,
    pub title: String,
    pub thumb: Thumb,
    pub category: u32,
    pub posted: String,
    /// Minimal MinimalPlus Compact Extended
    pub uploader: String,
    pub rating: f32,
    /// Compact Extended
    pub simple_tag_vec: Vec<String>,
    pub pages: u32,
    pub simple_language_opt: Option<String>,
    pub is_favorited: bool,
    pub favorite_slot_opt: Option<u32>,
    pub favorite_name_opt: Option<String>,
}

#[derive(Debug, PartialEq)]
pub struct GalleryInfoThumbnail {
    pub gid: u64,
    pub token: String,
    pub title: String,
    pub thumb: Thumb,
    pub category: u32,
    pub posted: String,
    pub rating: f32,
    pub pages: u32,
    pub simple_language_opt: Option<String>,
    pub is_favorited: bool,
    pub favorite_slot_opt: Option<u32>,
    pub favorite_name_opt: Option<String>,
}
