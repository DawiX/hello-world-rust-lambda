use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Response {
    pub message: String,
    pub buckets: Vec<String>,
}