use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct RateGallery {
    #[serde(alias = "rating_avg")]
    pub rating: f32,
    #[serde(alias = "rating_cnt")]
    pub rating_count: i32,
}
