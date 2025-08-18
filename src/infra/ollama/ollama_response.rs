use serde::Deserialize;

use crate::infra::ollama::OllamaResponseMessage;

#[derive(Debug, Deserialize)]
pub struct OllamaResponse {
    pub model: String,
    pub created_at: String,
    pub message: OllamaResponseMessage,
    pub done_reason: String,
    pub done: bool,
    pub total_duration: u64,
    pub load_duration: u64,
    pub prompt_eval_count: u32,
    pub prompt_eval_duration: u64,
    pub eval_count: u32,
    pub eval_duration: u64,
}
