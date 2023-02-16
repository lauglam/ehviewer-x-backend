#![allow(dead_code)]

use crate::CONCAT;
use crate::settings::{GallerySites, Settings};

#[derive(Debug, PartialEq, Clone)]
pub struct EhUrl {
    settings: Settings,
}

impl EhUrl {
    pub fn new(settings: Settings) -> EhUrl {
        EhUrl { settings }
    }

    pub fn gallery_detail(&self, gid: u64, token: &str, all_comment: bool, index_opt: Option<u32>) -> String {
        let prefix = format!("/g/{}/{}", gid, token);
        let mut prefix = match self.settings.site() {
            GallerySites::E => format!("{}{}", HOST_E, prefix),
            _ => format!("{}{}", HOST_EX, prefix),
        };

        if all_comment {
            prefix = prefix + "?hc=1";
        }

        if let Some(index) = index_opt {
            let suffix = if all_comment {
                format!("&p={}", index)
            } else {
                format!("?p={}", index)
            };

            prefix = prefix + &suffix;
        }

        prefix + "/"
    }

    pub fn gallery_multi_page_viewer(&self, gid: u64, token: &str) -> String {
        let suffix = format!("?mpv/{}/{}/", gid, token);
        match self.settings.site() {
            GallerySites::E => format!("{}{}", HOST_E, suffix),
            _ => format!("{}{}", HOST_EX, suffix),
        }
    }

    pub fn page(&self, gid: u64, index: u32, p_token: &str) -> String {
        let suffix = format!("s/{}/{}-{}", p_token, gid, (index + 1));
        match self.settings.site() {
            GallerySites::E => format!("{}{}", HOST_E, suffix),
            _ => format!("{}{}", HOST_EX, suffix),
        }
    }

    pub fn add_favorites(&self, gid: u64, token: &str) -> String {
        let suffix = format!(r#"gallerypopups.php?gid={}&t={}&act=addfav"#, gid, token);
        match self.settings.site() {
            GallerySites::E => format!("{}{}", HOST_E, suffix),
            _ => format!("{}{}", HOST_EX, suffix),
        }
    }

    pub fn download_archive(&self, gid: u64, token: &str, or: &str) -> String {
        let suffix = format!(r#"archiver.php?gid={}&token={}&or={}"#, gid, token, or);
        match self.settings.site() {
            GallerySites::E => format!("{}{}", HOST_E, suffix),
            _ => format!("{}{}", HOST_EX, suffix),
        }
    }

    pub fn favorites(&self) -> String {
        match self.settings.site() {
            GallerySites::E => String::from(URL_FAVORITES_E),
            _ => String::from(URL_FAVORITES_EX),
        }
    }

    pub fn api(&self) -> String {
        match self.settings.site() {
            GallerySites::E => String::from(API_E),
            _ => String::from(API_EX),
        }
    }

    pub fn referer(&self) -> String {
        match self.settings.site() {
            GallerySites::E => String::from(REFERER_E),
            _ => String::from(REFERER_EX),
        }
    }

    pub fn origin(&self) -> String {
        match self.settings.site() {
            GallerySites::E => String::from(ORIGIN_E),
            _ => String::from(ORIGIN_EX),
        }
    }

    pub fn my_tag(&self) -> String {
        match self.settings.site() {
            GallerySites::E => String::from(URL_MY_TAGS_E),
            _ => String::from(URL_MY_TAGS_EX),
        }
    }

    pub fn uconfig(&self) -> String {
        match self.settings.site() {
            GallerySites::E => String::from(URL_UCONFIG_E),
            _ => String::from(URL_UCONFIG_EX),
        }
    }

    pub fn tag_definition(&self, tag: &str) -> String {
        format!("https://ehwiki.org/wiki/{}", tag.replace(" ", "_"))
    }

    pub fn popular(&self) -> String {
        match self.settings.site() {
            GallerySites::E => String::from(URL_POPULAR_E),
            _ => String::from(URL_POPULAR_EX),
        }
    }

    pub fn image_search(&self) -> String {
        match self.settings.site() {
            GallerySites::E => String::from(URL_IMAGE_SEARCH_E),
            _ => String::from(URL_IMAGE_SEARCH_EX),
        }
    }

    pub fn watched(&self) -> String {
        match self.settings.site() {
            GallerySites::E => String::from(URL_WATCHED_E),
            _ => String::from(URL_WATCHED_EX),
        }
    }

    pub fn thumb_prefix(&self) -> String {
        // TODO ex
        String::from(URL_PREFIX_THUMB_E)
    }
}

pub const DOMAIN_EX: &str = "exhentai.org";
pub const DOMAIN_E: &str = "e-hentai.org";
pub const DOMAIN_LOFI: &str = "lofi.e-hentai.org";
pub const API_SIGN_IN: &str = "https://forums.e-hentai.org/index.php?act=Login&CODE=01";
pub const URL_FORUMS: &str = "https://forums.e-hentai.org/";

const HOST_EX: &str = CONCAT!("https://", DOMAIN_EX, "/");
const HOST_E: &str = CONCAT!("https://", DOMAIN_E, "/");

const API_E: &str = CONCAT!(HOST_E, "api.php");
const API_EX: &str = CONCAT!(HOST_EX, "api.php");

const URL_POPULAR_E: &str = "https://e-hentai.org/popular";
const URL_POPULAR_EX: &str = "https://exhentai.org/popular";

const URL_IMAGE_SEARCH_E: &str = "https://upload.e-hentai.org/image_lookup.php";
const URL_IMAGE_SEARCH_EX: &str = "https://exhentai.org/upload/image_lookup.php";

const URL_SIGN_IN: &str = "https://forums.e-hentai.org/index.php?act=Login";
const URL_REGISTER: &str = "https://forums.e-hentai.org/index.php?act=Reg&CODE=00";
const URL_FAVORITES_E: &str = CONCAT!(HOST_E, "favorites.php");
const URL_FAVORITES_EX: &str = CONCAT!(HOST_EX, "favorites.php");

const REFERER_EX: &str = CONCAT!("https://", DOMAIN_EX);
const REFERER_E: &str = CONCAT!("https://", DOMAIN_E);

const ORIGIN_EX: &str = REFERER_EX;
const ORIGIN_E: &str = REFERER_E;

const URL_UCONFIG_E: &str = CONCAT!(HOST_E, "uconfig.php");
const URL_UCONFIG_EX: &str = CONCAT!(HOST_EX, "uconfig.php");

const URL_MY_TAGS_E: &str = CONCAT!(HOST_E, "mytags");
const URL_MY_TAGS_EX: &str = CONCAT!(HOST_EX, "mytags");

const URL_WATCHED_E: &str = CONCAT!(HOST_E, "watched");
const URL_WATCHED_EX: &str = CONCAT!(HOST_EX, "watched");

const URL_PREFIX_THUMB_E: &str = "https://ehgt.org/";
// const URL_PREFIX_THUMB_EX: &str = "https://exhentai.org/t/";
