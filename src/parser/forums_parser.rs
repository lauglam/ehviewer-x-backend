use std::str::FromStr;
use visdom::Vis;
use crate::parser::{ATTRIBUTE_NOT_FOUND, ParseError};

#[derive(Debug, PartialEq)]
pub struct Forums {
    /// Links to user profile page.
    pub user_link: String,
}

impl FromStr for Forums {
    type Err = ParseError;

    /// ```html
    /// <div id="userlinks"><p class="home"><b>Logged in as:  <a href="https://forums.e-hentai.org/index.php?showuser=xxxxx">
    ///                                                            ^
    ///                                                            This is we looking for.
    /// ```
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let root = Vis::load(s)?;
        let user_link = root.find("#userlinks a");

        let href = user_link.attr("href").ok_or(ATTRIBUTE_NOT_FOUND)?;
        let user_link = href.to_string();

        Ok(Forums { user_link })
    }
}

#[cfg(test)]
mod tests {
    use crate::test_helper::read_test_file;
    use super::*;

    #[test]
    fn forums_parse_test() {
        let s = read_test_file("forums.html");
        assert_eq!(s.parse::<Forums>().is_ok(), true);
    }
}
