use std::str::FromStr;
use regex::Regex;
use crate::parser::{ParseError, REGEX_MATCH_FAILED};
use crate::structures::SignIn;


impl FromStr for SignIn {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let regex = Regex::new(NAME_PATTERN).unwrap();
        if let Some(cap) = regex.captures(s) {
            let username = String::from(&cap[1]);

            Ok(SignIn { username })
        } else {
            let regex = Regex::new(ERROR_PATTERN).unwrap();
            if let Some(cap) = regex.captures(s) {
                let error = String::from(
                    if let Some(m) = cap.get(1) {
                        m.as_str()
                    } else {
                        &cap[2]
                    }
                );

                Err(ParseError::FromServer(error))
            } else {
                Err(REGEX_MATCH_FAILED)
            }
        }
    }
}

const NAME_PATTERN: &str = "<p>You are now logged in as: (.+?)<";
const ERROR_PATTERN: &str = r#"(?:<h4>The error returned was:</h4>\s*<p>(.+?)</p>)|(?:<span class="postcolor">(.+?)</span>)"#;

#[cfg(test)]
mod tests {
    use crate::test_helper::read_test_file;
    use super::*;

    #[test]
    fn parse_test() {
        let s = read_test_file("sign_in_error.html");
        assert_eq!(s.parse::<SignIn>().is_err(), true);

        let s = read_test_file("sign_in_success.html");
        assert_eq!(s.parse::<SignIn>().is_ok(), true);
    }
}
