use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Clone)]
pub struct OllamaRequest {
    pub model: String,
    pub messages: Vec<OllamaMessage>,
    pub stream: bool,
}

#[derive(Debug, Serialize, Clone)]
pub struct OllamaMessage {
    pub role: String,
    pub content: String,
}

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

#[derive(Debug, Deserialize)]
pub struct OllamaResponseMessage {
    pub role: String,
    pub content: String,
}

impl OllamaRequest {
    pub fn new(model: String, content: String) -> Self {
        Self {
            model,
            messages: vec![OllamaMessage {
                role: "user".to_string(),
                content,
            }],
            stream: false,
        }
    }

    pub fn with_messages(model: String, messages: Vec<OllamaMessage>) -> Self {
        Self {
            model,
            messages,
            stream: false,
        }
    }
}

impl OllamaMessage {
    pub fn new(role: String, content: String) -> Self {
        Self { role, content }
    }

    pub fn user(content: String) -> Self {
        Self::new("user".to_string(), content)
    }

    pub fn assistant(content: String) -> Self {
        Self::new("assistant".to_string(), content)
    }
}