use std::str::FromStr;
use crate::parser::ParseError;
use crate::structures::VoteComment;

impl FromStr for VoteComment {
    type Err = ParseError;

    /// ```json
    /// {
    ///     "comment_id": 1253922,
    ///     "comment_score": -19,
    ///     "comment_vote": 0
    /// }
    /// ```
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(serde_json::from_str(s)?)
    }
}
