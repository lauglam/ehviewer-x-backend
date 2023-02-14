use crate::structures::{GalleryList, SearchNav};

#[derive(Debug, PartialEq)]
pub struct Favorite {
    pub search_nav: SearchNav,
    /// Size 10
    pub cat_vec: Vec<String>,
    /// Size 10
    pub count_vec: Vec<u32>,
    pub gallery_list: GalleryList,
}
