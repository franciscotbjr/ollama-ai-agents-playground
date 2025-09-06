use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct OllamaLoadResult {
    pub model: String,
    pub created_at: String,
    pub response: String,
    #[serde(default)]
    pub done: bool,
    pub done_reason: String,
}
