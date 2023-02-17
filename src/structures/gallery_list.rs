use crate::structures::{SearchNav, GalleryInfo};

#[derive(Debug, PartialEq)]
pub struct GalleryList {
    pub search_nav: SearchNav,
    pub gallery_info_vec: Vec<GalleryInfo>,
}
