use ollamars::ollama_response_message::OllamaResponseMessage;

use crate::agents::classifier::ClassificationResult;
use crate::agents::classifier::classification_result::OllamaIntentResponseParser;
use std::error::Error;
use std::fmt;

/// Error type for mapping operations
#[derive(Debug)]
pub enum MapperError {
    ParseError(String),
    InvalidContent(String),
    MissingData(String),
}

impl fmt::Display for MapperError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MapperError::ParseError(msg) => write!(f, "Parse error: {msg}"),
            MapperError::InvalidContent(msg) => write!(f, "Invalid content: {msg}"),
            MapperError::MissingData(msg) => write!(f, "Missing data: {msg}"),
        }
    }
}

impl Error for MapperError {}

/// Mapper trait for converting between types
pub trait Mapper<T, U> {
    type Error;

    fn map(source: T) -> Result<U, Self::Error>;
}

/// Implementation of mapper from OllamaResponseMessage to ClassificationResult
pub struct OllamaToClassificationMapper;

impl Mapper<&OllamaResponseMessage, ClassificationResult> for OllamaToClassificationMapper {
    type Error = MapperError;

    fn map(source: &OllamaResponseMessage) -> Result<ClassificationResult, Self::Error> {
        // Try to parse the structured content from the response
        let parsed_content = source
            .parsed_content(OllamaIntentResponseParser::default())
            .map_err(|e| MapperError::ParseError(e.to_string()))?;

        // Create ClassificationResult from the parsed content
        let result = ClassificationResult::new(parsed_content.intent, parsed_content.params);

        Ok(result)
    }
}

/// Convenience function for mapping
pub fn map_ollama_to_classification(
    response: &OllamaResponseMessage,
) -> Result<ClassificationResult, MapperError> {
    OllamaToClassificationMapper::map(response)
}

/// Extension trait for OllamaResponseMessage to provide convenient mapping
pub trait ToClassificationResult {
    fn to_classification_result(&self) -> Result<ClassificationResult, MapperError>;
}

impl ToClassificationResult for OllamaResponseMessage {
    fn to_classification_result(&self) -> Result<ClassificationResult, MapperError> {
        OllamaToClassificationMapper::map(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::agents::Intent;

    fn create_test_response_message(content: &str) -> OllamaResponseMessage {
        serde_json::from_str(&format!(
            r#"{{"role": "assistant", "content": "{}"}}"#,
            content.replace('"', r#"\""#).replace('\n', r#"\n"#)
        ))
        .unwrap()
    }

    #[test]
    fn test_map_valid_send_email_response() {
        let content = r#"```json
{
  "intent": "send_email",
  "params": {
    "recipient": "john@example.com",
    "message": "Meeting has been rescheduled"
  }
}
```"#;

        let response = create_test_response_message(content);
        let result = OllamaToClassificationMapper::map(&response).unwrap();

        assert_eq!(result.intent, Intent::SendEmail);
        assert_eq!(result.params.recipient(), Some("john@example.com"));
        assert_eq!(
            result.params.message(),
            Some("Meeting has been rescheduled")
        );
    }

    #[test]
    fn test_map_valid_schedule_meeting_response() {
        let content = r#"```json
{
  "intent": "schedule_meeting",
  "params": {
    "recipient": "alice@company.com",
    "message": "Let's schedule our quarterly review"
  }
}
```"#;

        let response = create_test_response_message(content);
        let result = OllamaToClassificationMapper::map(&response).unwrap();

        assert_eq!(result.intent, Intent::ScheduleMeeting);
        assert_eq!(result.params.recipient(), Some("alice@company.com"));
        assert_eq!(
            result.params.message(),
            Some("Let's schedule our quarterly review")
        );
    }

    #[test]
    fn test_map_no_action_response() {
        let content = r#"```json
{
  "intent": "no_action",
  "params": {
    "recipient": null,
    "message": null
  }
}
```"#;

        let response = create_test_response_message(content);
        let result = OllamaToClassificationMapper::map(&response).unwrap();

        assert_eq!(result.intent, Intent::NoAction);
        assert_eq!(result.params.recipient(), None);
        assert_eq!(result.params.message(), None);
    }

    #[test]
    fn test_map_plain_json_response() {
        let content = r#"{
  "intent": "send_email",
  "params": {
    "recipient": "plain@example.com",
    "message": "Plain JSON without markdown"
  }
}"#;

        let response = create_test_response_message(content);
        let result = OllamaToClassificationMapper::map(&response).unwrap();

        assert_eq!(result.intent, Intent::SendEmail);
        assert_eq!(result.params.recipient(), Some("plain@example.com"));
    }

    #[test]
    fn test_map_invalid_content() {
        let content = "This is not valid JSON content";
        let response = create_test_response_message(content);

        let result = OllamaToClassificationMapper::map(&response);
        assert!(result.is_err());

        if let Err(MapperError::ParseError(_)) = result {
            // Expected error type
        } else {
            panic!("Expected ParseError");
        }
    }

    #[test]
    fn test_map_malformed_json() {
        let content = r#"```json
{
  "intent": "send_email",
  "params": {
    "recipient": "test@example.com"
    // Missing closing braces
```"#;

        let response = create_test_response_message(content);
        let result = OllamaToClassificationMapper::map(&response);

        assert!(result.is_err());
    }

    #[test]
    fn test_convenience_function() {
        let content = r#"```json
{
  "intent": "send_email",
  "params": {
    "recipient": "convenience@test.com",
    "message": "Testing convenience function"
  }
}
```"#;

        let response = create_test_response_message(content);
        let result = map_ollama_to_classification(&response).unwrap();

        assert_eq!(result.intent, Intent::SendEmail);
        assert_eq!(result.params.recipient(), Some("convenience@test.com"));
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

        // Serialize back to JSON
        let json_string = result.to_json_string().unwrap();

        // Should contain the expected data
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
