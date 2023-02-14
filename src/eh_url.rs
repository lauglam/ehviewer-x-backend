#![allow(dead_code)]

use crate::CONCAT;
use crate::settings::{Settings};

#[derive(Debug, PartialEq, Clone)]
pub struct EhUrl {
    settings: Settings,
}

impl EhUrl {
    pub fn new(settings: Settings) -> EhUrl {
        EhUrl { settings }
    }
}

pub const SITE_E: usize = 0;
pub const SITE_EX: usize = 1;

pub const DOMAIN_EX: &str = "exhentai.org";
pub const DOMAIN_E: &str = "e-hentai.org";
pub const DOMAIN_LOFI: &str = "lofi.e-hentai.org";

pub const HOST_EX: &str = CONCAT!("https://", DOMAIN_EX, "/");
pub const HOST_E: &str = CONCAT!("https://", DOMAIN_E, "/");

pub const API_SIGN_IN: &str = "https://forums.e-hentai.org/index.php?act=Login&CODE=01";

pub const API_E: &str = CONCAT!(HOST_E, "api.php");
pub const API_EX: &str = CONCAT!(HOST_EX, "api.php");

pub const URL_POPULAR_E: &str = "https://e-hentai.org/popular";
pub const URL_POPULAR_EX: &str = "https://exhentai.org/popular";

pub const URL_IMAGE_SEARCH_E: &str = "https://upload.e-hentai.org/image_lookup.php";
pub const URL_IMAGE_SEARCH_EX: &str = "https://exhentai.org/upload/image_lookup.php";

pub const URL_SIGN_IN: &str = "https://forums.e-hentai.org/index.php?act=Login";
pub const URL_REGISTER: &str = "https://forums.e-hentai.org/index.php?act=Reg&CODE=00";
pub const URL_FAVORITES_E: &str = CONCAT!(HOST_E, "favorites.php");
pub const URL_FAVORITES_EX: &str = CONCAT!(HOST_EX, "favorites.php");
pub const URL_FORUMS: &str = "https://forums.e-hentai.org/";

pub const REFERER_EX: &str = CONCAT!("https://", DOMAIN_EX);
pub const REFERER_E: &str = CONCAT!("https://", DOMAIN_E);

pub const ORIGIN_EX: &str = REFERER_EX;
pub const ORIGIN_E: &str = REFERER_E;

pub const URL_UCONFIG_E: &str = CONCAT!(HOST_E, "uconfig.php");
pub const URL_UCONFIG_EX: &str = CONCAT!(HOST_EX, "uconfig.php");

pub const URL_MY_TAGS_E: &str = CONCAT!(HOST_E, "mytags");
pub const URL_MY_TAGS_EX: &str = CONCAT!(HOST_EX, "mytags");

pub const URL_WATCHED_E: &str = CONCAT!(HOST_E, "watched");
pub const URL_WATCHED_EX: &str = CONCAT!(HOST_EX, "watched");

const URL_PREFIX_THUMB_E: &str = "https://ehgt.org/";
// const URL_PREFIX_THUMB_EX: &str = "https://exhentai.org/t/";
