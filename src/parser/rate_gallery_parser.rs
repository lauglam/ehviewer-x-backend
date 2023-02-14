use std::str::FromStr;
use crate::parser::ParseError;
use crate::structures::RateGallery;

impl FromStr for RateGallery {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(serde_json::from_str(s)?)
    }
}
