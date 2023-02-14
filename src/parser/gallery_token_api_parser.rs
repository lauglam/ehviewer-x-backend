use std::str::FromStr;
use crate::parser::ParseError;
use crate::structures::GalleryTokenList;

impl FromStr for GalleryTokenList {
    type Err = ParseError;

    /// ```json
    /// {
    ///     "tokenlist": [
    ///         {
    ///             "gid": 2062874,
    ///             "token": "03037d8698"
    ///         }
    ///     ]
    /// }
    /// ```
    /// Or
    /// ```json
    /// {
    ///     "error": "maomao is moe~"
    /// }
    /// ```
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(serde_json::from_str(s)?)
    }
}

#[cfg(test)]
mod tests {
    use crate::structures::GalleryToken;
    use super::*;

    #[test]
    fn parse_test() {
        let json = r#"
            {
                "tokenlist": [
                    {
                        "gid": 2062874,
                        "token": "03037d8698"
                    }
                ]
            }
        "#;

        assert_eq!(json.parse::<GalleryTokenList>().unwrap(), GalleryTokenList {
            token_vec_opt: Some(vec![GalleryToken {
                gid: 2062874,
                token: String::from("03037d8698"),
            }]),
            error_opt: None,
        });

        let json = r#"
            {
                "error": "maomao is moe~"
            }
        "#;

        assert_eq!(json.parse::<GalleryTokenList>().unwrap(), GalleryTokenList {
            token_vec_opt: None,
            error_opt: Some(String::from(r#"maomao is moe~"#)),
        });
    }
}
