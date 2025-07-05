use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CommonRs<T> {
    pub message: String,
    pub code: String,
    pub data: T,
}