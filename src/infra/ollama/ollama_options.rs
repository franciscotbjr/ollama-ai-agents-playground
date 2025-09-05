use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct OllamaOptions {
    pub temperature: f32,
}

impl OllamaOptions {
    pub fn new(temperature: f32) -> Self {
        Self { temperature }
    }
}