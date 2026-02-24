use ollama_oxide::ResponseMessage;

use super::classification_result::ClassificationResult;
use super::from_markdown_json::FromMarkdownJson;

#[derive(Default)]
pub struct OllamaIntentResponseParser {}

impl OllamaIntentResponseParser {
    /// Extracts JSON content from markdown code block
    fn extract_json_from_markdown(content: &str) -> Result<String, Box<dyn std::error::Error>> {
        // Find the start and end of the JSON code block
        if let Some(start) = content.find("```json") {
            let after_start = &content[start + 7..]; // Skip "```json"
            if let Some(end) = after_start.find("```") {
                let json_content = &after_start[..end].trim();
                return Ok(json_content.to_string());
            }
        }

        // Fallback: try to find JSON without markdown markers
        if content.trim().starts_with('{') && content.trim().ends_with('}') {
            return Ok(content.trim().to_string());
        }

        Err(format!("Could not extract JSON from content: {content}").into())
    }
}

impl FromMarkdownJson<ClassificationResult> for OllamaIntentResponseParser {
    /// Extracts JSON from ```json ... ``` markdown format and parses it
    fn from_markdown_text(
        markdown_text: &str,
    ) -> Result<ClassificationResult, Box<dyn std::error::Error>> {
        let json_content = OllamaIntentResponseParser::extract_json_from_markdown(markdown_text)?;
        let parsed = serde_json::from_str(&json_content)?;
        Ok(parsed)
    }
}

impl FromMarkdownJson<ClassificationResult> for ResponseMessage {
    fn from_markdown_text(
        markdown_text: &str,
    ) -> Result<ClassificationResult, Box<dyn std::error::Error>> {
        OllamaIntentResponseParser::from_markdown_text(markdown_text)
    }
}
