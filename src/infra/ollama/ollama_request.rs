use serde::Serialize;

use crate::infra::ollama::OllamaMessage;

#[derive(Debug, Serialize, Clone)]
pub struct OllamaRequest {
    pub model: String,
    pub messages: Vec<OllamaMessage>,
    pub stream: bool,
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