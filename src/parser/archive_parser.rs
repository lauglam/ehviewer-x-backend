use std::str::FromStr;
use regex::Regex;
use crate::{
    structures::{Archive, ArchiveItem},
    parser::{ParseError, REGEX_MATCH_FAILED, unescape::unescape},
};

impl FromStr for Archive {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let regex = Regex::new(PATTERN_FORM).unwrap();
        let captures = regex.captures(s).ok_or(REGEX_MATCH_FAILED)?;
        let or = String::from(&captures[1]);

        let regex = Regex::new(PATTERN_ARCHIVE).unwrap();
        let mut items = Vec::new();

        for cap in regex.captures_iter(s) {
            let res = String::from(unescape(&cap[1]));
            let name = String::from(unescape(&cap[2]));

            items.push(ArchiveItem {
                res,
                name,
            });
        }

        Ok(Archive { or, items })
    }
}

const PATTERN_FORM: &str = r#"<form id="hathdl_form" action="[^"]*?or=([^="]*?)" method="post">"#;
const PATTERN_ARCHIVE: &str = r#"<a href="[^"]*" onclick="return do_hathdl\('([0-9]+|org)'\)">([^<]+)</a>"#;

#[cfg(test)]
mod tests {
    use crate::test_helper::read_test_file;
    use super::*;

    #[test]
    fn parse_test() {
        let s = read_test_file("archive.html");
        assert_eq!(s.parse::<Archive>().is_ok(), true);
    }
}
