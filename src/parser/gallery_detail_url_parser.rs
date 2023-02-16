use std::str::FromStr;
use regex::Regex;
use crate::{CONCAT, eh_url, parser::{ParseError, REGEX_MATCH_FAILED}, structures::GalleryDetailUrl};

impl FromStr for GalleryDetailUrl {
    type Err = ParseError;

    /// ```text
    /// https://e-hentai.org/g/2455981/acc72caed0/
    /// ```
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let regex = Regex::new(URL_STRICT_PATTERN).unwrap();

        let captures = regex.captures(s).ok_or(REGEX_MATCH_FAILED)?;
        let gid = captures[1].parse::<u64>()?;
        let token = String::from(&captures[2]);

        Ok(GalleryDetailUrl { gid, token })
    }
}

const URL_STRICT_PATTERN: &str = CONCAT!("https?://(?:", eh_url::DOMAIN_EX, "|", eh_url::DOMAIN_E, "|", eh_url::DOMAIN_LOFI, ")/(?:g|mpv)/(\\d+)/([0-9a-f]{10})");
// const URL_PATTERN: &str = "(\\d+)/([0-9a-f]{10})(?:[^0-9a-f]|$)";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_test() {
        let url = "https://e-hentai.org/g/2455981/acc72caed0/";
        assert_eq!(url.parse::<GalleryDetailUrl>().is_ok(), true);
    }
}
