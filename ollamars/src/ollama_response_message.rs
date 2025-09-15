use crate::from_markdown_json::FromMarkdownJson;

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
    pub fn parsed_content<T, P>(&self, parser: P) -> Result<T, Box<dyn std::error::Error>>
    where
        P: FromMarkdownJson<T>,
    {
        P::from_markdown_text(&self.raw_content)
    }
}
