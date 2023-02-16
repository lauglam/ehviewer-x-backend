use std::str::FromStr;
use regex::Regex;
use crate::{parser::{ParseError, REGEX_MATCH_FAILED}, structures::Torrent};

impl FromStr for Torrent {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let regex = Regex::new(PATTERN_TORRENT).unwrap();
        let captures = regex.captures(s).ok_or(REGEX_MATCH_FAILED)?;

        let download_url = String::from(&captures[1]);
        let filename = String::from(&captures[2]);

        Ok(Torrent {
            filename,
            download_url,
        })
    }
}

const PATTERN_TORRENT: &str = r#"<td colspan="5"> &nbsp; <a href=".*" onclick="sument.location='([^"]+)'[^<]+>([^<]+)</a></td>"#;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_test() {
        let ele = r#"
            <tr>
                <td colspan="5"> &nbsp; <a href="https://ehtracker.org/get/2257278/9a16691657fb9ec9ad124298af12eaaf86fa614c.torrent" onclick="sument.location='https://ehtracker.org/get/xxxx/xxxx.torrent?p=xxxx'; return false">xxxx.zip</a></td>
            </tr>
        "#;

        assert_eq!(ele.parse::<Torrent>().unwrap(), Torrent {
            filename: String::from(r#"xxxx.zip"#),
            download_url: String::from("https://ehtracker.org/get/xxxx/xxxx.torrent?p=xxxx"),
        });
    }
}
