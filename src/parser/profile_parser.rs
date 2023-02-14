use std::str::FromStr;
use visdom::Vis;
use crate::eh_url;
use crate::parser::{ATTRIBUTE_NOT_FOUND, ParseError};
use crate::structures::Profile;

impl FromStr for Profile {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let root = Vis::load(s)?;
        let display_name = root.find("#profilename > font");
        let display_name = display_name.text();

        let avatar = root.find(r#".ipbtable img"#);
        let avatar = avatar.attr("src").ok_or(ATTRIBUTE_NOT_FOUND)?;
        let mut avatar = avatar.to_string();
        if !avatar.starts_with("http") {
            avatar = format!("{}{}", eh_url::URL_FORUMS, avatar);
        }

        Ok(Profile {
            display_name,
            avatar,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helper::read_test_file;
    use crate::eh_url;

    #[test]
    fn forums_parse_test() {
        let s = read_test_file("profile.html");

        assert_eq!(s.parse::<Profile>().unwrap(), Profile {
            display_name: String::from(r#"xxxx"#),
            avatar: format!("{}{}", eh_url::URL_FORUMS, "style_images/ambience/warn0.gif"),
        });
    }
}
