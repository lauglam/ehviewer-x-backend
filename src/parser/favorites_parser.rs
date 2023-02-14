use std::str::FromStr;
use visdom::Vis;
use crate::parser::{ParseError, SIGN_IN_REQUIRED};
use crate::structures::Favorite;

impl FromStr for Favorite {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.contains("This page requires you to log on.</p>") {
            return Err(SIGN_IN_REQUIRED);
        }

        let mut cat_vec = Vec::new();
        let mut count_vec = Vec::new();

        let root = Vis::load(s)?;
        // skip last one: <div class="fp fps"...
        let fps = root.find(".ido [class=fp]");

        assert_eq!(fps.length(), 10);
        for fp in fps {
            let children = fp.children();
            let count = children.eq(0);
            let cat = children.eq(2);
            count_vec.push(count.text().parse::<u32>()?);
            cat_vec.push(cat.text());
        }

        let nav = root.find(r#".searchnav"#).eq(0);
        let search_nav = nav.outer_html().parse()?;
        let gallery_list = s.parse()?;

        Ok(Favorite {
            search_nav,
            cat_vec,
            count_vec,
            gallery_list,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::test_helper::read_test_file;
    use super::*;

    #[test]
    fn parse_sign_in_required_test() {
        let s = read_test_file("sign_in_required.html");
        assert_eq!(s.parse::<Favorite>().is_err(), true);
    }

    #[test]
    fn parse_test() {
        let s = read_test_file("favorites.html");
        assert_eq!(s.parse::<Favorite>().is_ok(), true);
    }
}
