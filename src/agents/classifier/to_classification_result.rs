use ollama_oxide::ResponseMessage;

use super::classification_result::ClassificationResult;
use super::mapper::Mapper;
use super::mapper_error::MapperError;
use super::ollama_to_classification_mapper::OllamaToClassificationMapper;

/// Extension trait for ResponseMessage to provide convenient mapping
pub trait ToClassificationResult {
    fn to_classification_result(&self) -> Result<ClassificationResult, MapperError>;
}

impl ToClassificationResult for ResponseMessage {
    fn to_classification_result(&self) -> Result<ClassificationResult, MapperError> {
        OllamaToClassificationMapper::map(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::agents::Intent;

    fn create_test_response_message(content: &str) -> ResponseMessage {
        serde_json::from_str(&format!(
            r#"{{"role": "assistant", "content": "{}"}}"#,
            content.replace('"', r#"\""#).replace('\n', r#"\n"#)
        ))
        .unwrap()
    }

    #[test]
    fn test_extension_trait() {
        let content = r#"```json
{
  "intent": "schedule_meeting",
  "params": {
    "recipient": "trait@test.com",
    "message": "Testing extension trait"
  }
}
```"#;

        let response = create_test_response_message(content);
        let result = response.to_classification_result().unwrap();

        assert_eq!(result.intent, Intent::ScheduleMeeting);
        assert_eq!(result.params.recipient(), Some("trait@test.com"));
    }

    #[test]
    fn test_roundtrip_serialization() {
        let content = r#"```json
{
  "intent": "send_email",
  "params": {
    "recipient": "roundtrip@test.com",
    "message": "Testing roundtrip"
  }
}
```"#;

        let response = create_test_response_message(content);
        let result = response.to_classification_result().unwrap();

        let json_string = result.to_json_string().unwrap();

        assert!(json_string.contains("send_email"));
        assert!(json_string.contains("roundtrip@test.com"));
    }

    #[test]
    fn test_unicode_content_mapping() {
        let content = r#"```json
{
  "intent": "send_email",
  "params": {
    "recipient": "用户@example.com",
    "message": "Hello 世界! 🌍"
  }
}
```"#;

        let response = create_test_response_message(content);
        let result = response.to_classification_result().unwrap();

        assert_eq!(result.params.recipient(), Some("用户@example.com"));
        assert_eq!(result.params.message(), Some("Hello 世界! 🌍"));
    }

    #[test]
    fn test_partial_params() {
        let content = r#"```json
{
  "intent": "send_email",
  "params": {
    "recipient": "partial@test.com",
    "message": null
  }
}
```"#;

        let response = create_test_response_message(content);
        let result = response.to_classification_result().unwrap();

        assert_eq!(result.intent, Intent::SendEmail);
        assert_eq!(result.params.recipient(), Some("partial@test.com"));
        assert_eq!(result.params.message(), None);
    }
}
