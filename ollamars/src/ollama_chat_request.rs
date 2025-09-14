use serde::{Deserialize, Serialize};

use crate::{ollama_chat::OllamaChat, ollama_options::OllamaOptions};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct OllamaChatRequest {
    pub model: String,
    pub messages: Vec<OllamaChat>,
    pub stream: bool,
    #[serde(default)]
    pub think: bool,
    pub options: Option<OllamaOptions>,
}

impl OllamaChatRequest {
    pub fn new(model: String, messages: Vec<OllamaChat>, options: Option<OllamaOptions>) -> Self {
        Self {
            model,
            messages,
            stream: false,
            think: false,
            options,
        }
    }

    pub fn with_messages(
        model: String,
        messages: Vec<OllamaChat>,
        options: Option<OllamaOptions>,
    ) -> Self {
        Self {
            model,
            messages,
            stream: false,
            think: false,
            options,
        }
    }
}
