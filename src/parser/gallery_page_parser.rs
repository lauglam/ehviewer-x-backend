use std::str::FromStr;
use regex::Regex;
use crate::{parser::{ParseError, REGEX_MATCH_FAILED, unescape::unescape}, structures::GalleryPage};

impl FromStr for GalleryPage {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let regex = Regex::new(PATTERN_IMAGE_URL).unwrap();
        let captures = regex.captures(s).ok_or(REGEX_MATCH_FAILED)?;
        let image_url = String::from(&captures[1]);

        let regex = Regex::new(PATTERN_SKIP_HATH_KEY).unwrap();
        let captures = regex.captures(s).ok_or(REGEX_MATCH_FAILED)?;
        let skip_hath_key = String::from(&captures[1]);

        let regex = Regex::new(PATTERN_ORIGIN_IMAGE_URL).unwrap();
        let captures = regex.captures(s).ok_or(REGEX_MATCH_FAILED)?;
        let origin_image_url = format!("{}{}{}", &captures[1], r#"fullimg.php"#, unescape(&captures[2]));

        let regex = Regex::new(PATTERN_SHOW_KEY).unwrap();
        let captures = regex.captures(s).ok_or(REGEX_MATCH_FAILED)?;
        let show_key = String::from(&captures[1]);

        Ok(GalleryPage {
            image_url,
            skip_hath_key,
            origin_image_url,
            show_key,
        })
    }
}

const PATTERN_IMAGE_URL: &str = r#"<img[^>]*src="([^"]+)" style"#;
const PATTERN_SKIP_HATH_KEY: &str = r#"onclick="return nl\('([^\)]+)'\)"#;
const PATTERN_ORIGIN_IMAGE_URL: &str = r#"<a href=\"([^\"]+)fullimg.php([^\"]+)\">"#;
// TODO Not sure about the size of show keys
const PATTERN_SHOW_KEY: &str = r#"var showkey="([0-9a-z]+)";"#;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_test() {}
}
