use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct OllamaLoadResult {
    model: String,
    created_at: String,
    response: String,
    #[serde(default)]
    done: bool,
    #[serde(default)]
    done_reason: bool,
}
