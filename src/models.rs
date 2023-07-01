use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct StatusResponse {
    pub status: String,
    pub passed: Vec<String>,
    pub failed: Vec<String>,
}

#[derive(Deserialize, Serialize)]
pub struct LanguageMetadata {
    pub language: String,
    pub speed_index: f32,
    pub extension: String,
    pub aliases: Vec<String>,
}
