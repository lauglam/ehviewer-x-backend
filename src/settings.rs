#[derive(Debug, PartialEq, Clone)]
pub struct Settings {
    site: GallerySites,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            site: GallerySites::E,
        }
    }
}

impl Settings {
    pub fn new() -> Settings {
        Settings::default()
    }

    pub fn site(&self) -> GallerySites {
        self.site
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum GallerySites {
    E,
    EX,
}
