use std::str::FromStr;
use regex::Regex;
use crate::parser::{ParseError, REGEX_MATCH_FAILED};
use crate::structures::Thumb;

impl FromStr for Thumb {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let regex = Regex::new(PATTERN_THUMB).unwrap();
        let captures = regex.captures(s).ok_or(REGEX_MATCH_FAILED)?;

        let height = captures[1].parse()?;
        let width = captures[2].parse()?;
        let src = String::from(&captures[3]);

        Ok(Thumb {
            src,
            width,
            height,
        })
    }
}

const PATTERN_THUMB: &str = r#"<img[^>]*style="height:(\d+)px;width:(\d+)px[^"]*"[^>]*src="([^"]+)""#;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_test() {
        let img = r#"<img style="height:376px;width:250px;top:-18px" alt="[Pixiv] Moca (7010167) [AI Generated]" title="[Pixiv] Moca (7010167) [AI Generated]" src="https://ehgt.org/08/be/08be4188d5b91a484fc7eeb2a952f5d7eeeec5a3-463202-512-768-png_250.jpg">"#;
        assert_eq!(img.parse::<Thumb>().is_ok(), true);
    }
}
