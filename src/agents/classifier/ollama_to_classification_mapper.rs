use ollama_oxide::ResponseMessage;

use super::classification_result::ClassificationResult;
use super::from_markdown_json::FromMarkdownJson;
use super::mapper::Mapper;
use super::mapper_error::MapperError;

/// Implementation of mapper from ResponseMessage to ClassificationResult
pub struct OllamaToClassificationMapper;

impl Mapper<&ResponseMessage, ClassificationResult> for OllamaToClassificationMapper {
    type Error = MapperError;

    fn map(source: &ResponseMessage) -> Result<ClassificationResult, Self::Error> {
        let parsed_content =
            ResponseMessage::from_markdown_text(&source.content().unwrap())
                .map_err(|e| MapperError::ParseError(e.to_string()))?;

        let result = ClassificationResult::new(parsed_content.intent, parsed_content.params);

        Ok(result)
    }
}

/// Convenience function for mapping
pub fn map_ollama_to_classification(
    response: &ResponseMessage,
) -> Result<ClassificationResult, MapperError> {
    OllamaToClassificationMapper::map(response)
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
}
