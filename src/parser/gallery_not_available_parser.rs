use std::str::FromStr;
use visdom::Vis;
use crate::parser::ParseError;
use crate::structures::GalleryNotAvailable;

impl FromStr for GalleryNotAvailable {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let root = Vis::load(s)?;
        let p = root.find(".d p:first-child");
        let error = p.text();

        Ok(GalleryNotAvailable { error })
    }
}

#[cfg(test)]
mod tests {
    use crate::test_helper::read_test_file;
    use super::*;

    #[test]
    fn parse_test() {
        let s = read_test_file("gallery_not_available.html");
        assert_eq!(s.parse::<GalleryNotAvailable>().unwrap(), GalleryNotAvailable {
            error: String::from("This gallery_list has been removed or is unavailable.")
        })
    }
}
