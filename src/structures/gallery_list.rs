use crate::structures::{GalleryInfoSet, SearchNav};

#[derive(Debug, PartialEq)]
pub struct GalleryList {
    pub search_nav: SearchNav,
    pub gallery_info_set: GalleryInfoSet,
}
