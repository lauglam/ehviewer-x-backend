use std::str::FromStr;
use crate::{parser::{ParseError, input::Input}, structures::GalleryMultiPageViewerPToken};

impl FromStr for GalleryMultiPageViewerPToken {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut input = Input::new(s);
        let bgn = input.find_str(PREFIX).unwrap();
        input.set_cursor(bgn);
        let end = input.find(';').unwrap();

        let bgn = bgn + PREFIX.len() + 16;
        let image_list = input.get_string(bgn, end)?;
        let image_vec = serde_json::from_str::<Vec<String>>(&image_list)?;

        Ok(GalleryMultiPageViewerPToken {
            image_vec,
        })
    }
}

const PREFIX: &str = r#"var imagelist = "#;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_test() {}
}
