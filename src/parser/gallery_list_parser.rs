use std::str::FromStr;
use visdom::Vis;
use crate::{
    parser::ParseError,
    structures::{GalleryList, SearchNav, GalleryInfo},
};

impl FromStr for GalleryList {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let root = Vis::load(s)?;
        let nav = root.find(r#".searchnav"#).eq(0);
        let search_nav = nav.outer_html().parse::<SearchNav>()?;

        let selector = r#".searchnav select[onchange*=inline_set] > option[selected]"#;
        let selected = root.find(selector).last();

        let itg = root.find(".itg");
        let children = match selected.text().as_str() {
            "Minimal" | "Minimal+" | "Compact" => {
                let mut children = itg.children("tr").slice(1..);
                // if it is not log in, that 14th element of the array is an advertisement
                if children.length() > 25 {
                    children = children.slice(..13).add(children.slice(14..));
                }

                assert_eq!(children.length(), 25);
                children
            }
            "Extended" => {
                let mut children = itg.children("tr");
                // if it is not log in, that 14th element of the array is an advertisement
                if children.length() > 25 {
                    children = children.slice(..13).add(children.slice(14..));
                }

                assert_eq!(children.length(), 25);
                children
            }
            "Thumbnail" => {
                let children = itg.children(".gl1t");
                assert_eq!(children.length(), 25);
                children
            }
            _ => unreachable!(),
        };

        let mut gallery_info_vec = Vec::new();
        for child in children {
            gallery_info_vec.push(child.outer_html().parse::<GalleryInfo>()?);
        }

        Ok(GalleryList { search_nav, gallery_info_vec })
    }
}


#[cfg(test)]
mod tests {
    use crate::test_helper::read_test_file;
    use super::*;

    #[test]
    fn parse_gallery_list_test() {
        let s = read_test_file("gallery_list_minimal.html");
        let result = s.parse::<GalleryList>();

        let s = read_test_file("gallery_list_minimal_plus.html");
        let result = s.parse::<GalleryList>();

        let s = read_test_file("gallery_list_compact.html");
        let result = s.parse::<GalleryList>();

        let s = read_test_file("gallery_list_extended.html");
        let result = s.parse::<GalleryList>();

        let s = read_test_file("gallery_list_thumbnail.html");
        let result = s.parse::<GalleryList>();
    }
}
