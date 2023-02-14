use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct VoteComment {
    #[serde(alias = r#"comment_id"#)]
    pub id: u64,
    #[serde(alias = r#"comment_score"#)]
    pub score: i32,
    #[serde(alias = r#"comment_vote"#)]
    pub vote: u32,
    // TODO need expect_vote?
    // pub expect_vote: u32,
}
