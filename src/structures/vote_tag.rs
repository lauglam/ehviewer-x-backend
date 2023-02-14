use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct VoteTag {
    pub error: String,
}
