use super::ollama_intent_response_content::OllamaIntentResponseContent;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct OllamaResponseMessage {
    pub role: String,
    #[serde(rename = "content")]
    raw_content: String,
}

impl OllamaResponseMessage {
    /// Returns the raw content string as received from Ollama
    pub fn raw_content(&self) -> &str {
        &self.raw_content
    }

    /// Parses the content to extract structured JSON data
    pub fn parsed_content(
        &self,
    ) -> Result<T, Box<dyn std::error::Error>> {
        T::from_markdown_json(&self.raw_content)
    }

    /// Convenience method to get content, trying parsed first, fallback to raw
    pub fn content(&self) -> String {
        match self.parsed_content() {
            Ok(parsed) => parsed
                .to_json_string()
                .unwrap_or_else(|_| self.raw_content.clone()),
            Err(_) => self.raw_content.clone(),
        }
    }
}
