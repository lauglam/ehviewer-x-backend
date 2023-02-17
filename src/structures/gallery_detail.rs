use std::collections::HashMap;
use crate::structures::{GalleryIdentity, GalleryTagGroup};

#[derive(Debug, PartialEq)]
pub struct GalleryDetail {
    pub identity: GalleryIdentity,
    /// If you are not log in, that value is -1.
    pub api_uid: i64,
    pub api_key: String,
    pub torrent_count: u32,
    pub torrent_url: String,
    pub archive_url: String,
    pub thumb: String,
    pub newer_version_map_opt: Option<HashMap<String, GalleryIdentity>>,
    pub is_favorited: bool,
    pub favorite_name_opt: Option<String>,
    pub favorite_slot_opt: Option<u32>,
    pub rating_count: u32,
    pub tag_group_vec: Vec<GalleryTagGroup>,
    pub comment_list: GalleryCommentList,
    pub preview_pages: u32,
    pub preview_set: GalleryPreviewSet,
    pub url: String,
    pub title: String,
    pub title_jpn: String,
    pub category: u32,
    pub uploader: String,
    pub rating_opt: Option<f32>,
    pub detail: GalleryDetailDetail,
}

#[derive(Debug, PartialEq)]
pub struct GalleryComment {
    /// 0 for uploader comment. cannot vote.
    pub id: u64,
    /// uploader comment is `None`.
    pub score_opt: Option<u32>,
    pub editable: bool,
    pub vote_up_able: bool,
    pub vote_up_ed: bool,
    pub vote_down_able: bool,
    pub vote_down_ed: bool,
    pub is_uploader: bool,
    /// uploader comment is `None`.
    pub vote_state_opt: Option<String>,
    pub posted_timestamp: i64,
    pub user: String,
    pub comment: String,
    pub last_edited_timestamp_opt: Option<i64>,
}

#[derive(Debug, PartialEq)]
pub struct GalleryCommentList {
    pub comment_vec: Vec<GalleryComment>,
    pub has_more: bool,
}

#[derive(Debug, PartialEq)]
pub struct GalleryDetailDetail {
    pub posted: String,
    pub parent_opt: Option<String>,
    pub visible: String,
    pub language: String,
    pub file_size: String,
    pub pages: u32,
    pub favorite_count: u32,
}

#[derive(Debug, PartialEq)]
pub struct GalleryPreviewLarge {
    pub position: u32,
    pub filename: String,
    pub page_url: String,
    pub image_url: String,
}

#[derive(Debug, PartialEq)]
pub struct GalleryPreviewMedium {
    pub position: u32,
    pub filename: String,
    pub page_url: String,
    pub image_url: String,
    /// in the medium, the preview is a mosaic of 20 images.
    pub offset_x: u32,
    pub offset_y: u32,
    pub clip_width: u32,
    pub clip_height: u32,
}

#[derive(Debug, PartialEq)]
pub enum GalleryPreviewSet {
    Large(Vec<GalleryPreviewLarge>),
    Medium(Vec<GalleryPreviewMedium>),
}
