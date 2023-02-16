use std::str::FromStr;
use regex::Regex;
use crate::{parser::{ParseError, REGEX_MATCH_FAILED}, structures::Rating};

impl FromStr for Rating {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let reg = Regex::new(PATTERN_RATING).unwrap();
        let mut n1 = i32::MIN;
        let mut n2 = i32::MIN;

        let mut value = 5 as f32;
        let mut ms = reg.find_iter(s);
        if let Some(m) = ms.next() {
            n1 = m.as_str().replace("px", "").parse::<i32>()?;
        }

        if let Some(m) = ms.next() {
            n2 = m.as_str().replace("px", "").parse::<i32>()?;
        }

        if n1 != i32::MIN && n2 != i32::MIN {
            value -= (n1 / 16) as f32;
            if n2 == 21 {
                value -= 0.5 as f32;
            }

            Ok(Rating { value })
        } else {
            Err(REGEX_MATCH_FAILED)
        }
    }
}

const PATTERN_RATING: &str = r#"\d+px"#;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_rating_test() {
        let rating_style = "background-position:0px -21px;opacity:0.53333333333333";
        assert_eq!(rating_style.parse::<Rating>().unwrap().value, 4.5 as f32);
    }
}
