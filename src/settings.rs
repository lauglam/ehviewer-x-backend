#[derive(Debug, PartialEq, Clone)]
pub struct Settings {
    gallery_site: GallerySites,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            gallery_site: GallerySites::E,
        }
    }
}

impl Settings {
    pub fn gallery_site(&self) -> GallerySites {
        self.gallery_site
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum GallerySites {
    E,
    EX,
}
