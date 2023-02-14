use std::str::FromStr;
use crate::parser::ParseError;
use crate::structures::VoteTag;

impl FromStr for VoteTag {
    type Err = ParseError;

    /// ```json
    /// {
    ///     "error": "The tag \"neko\" is not allowed. Use character:neko or artist:neko"
    /// }
    /// ```
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(serde_json::from_str(s)?)
    }
}
