use serde::{Deserialize, Serialize};

use crate::infra::ollama::{OllamaChat, OllamaOptions};

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
    pub fn new(model: String, content: String, options: Option<OllamaOptions>) -> Self {
        Self {
            model,
            messages: vec![OllamaChat {
                role: "user".to_string(),
                content,
            }],
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

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_ollama_request_new() {
        let request = OllamaChatRequest::new(
            "llama2".to_string(),
            "Hello world".to_string(),
            Some(OllamaOptions { temperature: 0.0 }),
        );

        assert_eq!(request.model, "llama2");
        assert_eq!(request.messages.len(), 1);
        assert_eq!(request.messages[0].role, "user");
        assert_eq!(request.messages[0].content, "Hello world");
        assert_eq!(request.stream, false);
        assert_eq!(request.think, false);
        assert_eq!(request.options.unwrap().temperature, 0.0);
    }

    #[test]
    fn test_ollama_request_with_messages() {
        let messages = vec![
            OllamaChat::user("What is AI?".to_string()),
            OllamaChat::assistant("AI is artificial intelligence.".to_string()),
        ];
        let request = OllamaChatRequest::with_messages(
            "llama2".to_string(),
            messages.clone(),
            Some(OllamaOptions { temperature: 0.0 }),
        );

        assert_eq!(request.model, "llama2");
        assert_eq!(request.messages, messages);
        assert_eq!(request.stream, false);
        assert_eq!(request.think, false);
        assert_eq!(request.options.unwrap().temperature, 0.0);
    }

    #[test]
    fn test_ollama_request_serialization() {
        let request = OllamaChatRequest::new(
            "llama2".to_string(),
            "Test message".to_string(),
            Some(OllamaOptions { temperature: 0.0 }),
        );
        let json = serde_json::to_string(&request).expect("Serialization should succeed");

        let expected_json = r#"{"model":"llama2","messages":[{"role":"user","content":"Test message"}],"stream":false,"think":false,"options":{"temperature":0.0}}"#;
        assert_eq!(json, expected_json);
    }

    #[test]
    fn test_ollama_request_serialization_with_multiple_messages() {
        let messages = vec![
            OllamaChat::user("Question 1".to_string()),
            OllamaChat::assistant("Answer 1".to_string()),
            OllamaChat::user("Question 2".to_string()),
        ];
        let request = OllamaChatRequest::with_messages(
            "gpt-3.5".to_string(),
            messages,
            Some(OllamaOptions { temperature: 0.0 }),
        );
        let json = serde_json::to_string(&request).expect("Serialization should succeed");

        let expected_json = r#"{"model":"gpt-3.5","messages":[{"role":"user","content":"Question 1"},{"role":"assistant","content":"Answer 1"},{"role":"user","content":"Question 2"}],"stream":false,"think":false,"options":{"temperature":0.0}}"#;
        assert_eq!(json, expected_json);
    }

    #[test]
    fn test_ollama_request_deserialization() {
        let json = r#"{"model":"llama2","messages":[{"role":"user","content":"Test message"}],"stream":false}"#;
        let request: OllamaChatRequest =
            serde_json::from_str(json).expect("Deserialization should succeed");

        assert_eq!(request.model, "llama2");
        assert_eq!(request.messages.len(), 1);
        assert_eq!(request.messages[0].role, "user");
        assert_eq!(request.messages[0].content, "Test message");
        assert_eq!(request.stream, false);
        assert_eq!(request.think, false); // Default value when not present
    }

    #[test]
    fn test_ollama_request_deserialization_with_multiple_messages() {
        let json = r#"{"model":"claude","messages":[{"role":"user","content":"Hello"},{"role":"assistant","content":"Hi there!"},{"role":"user","content":"How are you?"}],"stream":true}"#;
        let request: OllamaChatRequest =
            serde_json::from_str(json).expect("Deserialization should succeed");

        assert_eq!(request.model, "claude");
        assert_eq!(request.messages.len(), 3);
        assert_eq!(request.messages[0].role, "user");
        assert_eq!(request.messages[0].content, "Hello");
        assert_eq!(request.messages[1].role, "assistant");
        assert_eq!(request.messages[1].content, "Hi there!");
        assert_eq!(request.messages[2].role, "user");
        assert_eq!(request.messages[2].content, "How are you?");
        assert_eq!(request.stream, true);
        assert_eq!(request.think, false); // Default value when not present
    }

    #[test]
    fn test_ollama_request_roundtrip_serialization() {
        let original = OllamaChatRequest::new(
            "test-model".to_string(),
            "Test content".to_string(),
            Some(OllamaOptions { temperature: 0.0 }),
        );
        let json = serde_json::to_string(&original).expect("Serialization should succeed");
        let deserialized: OllamaChatRequest =
            serde_json::from_str(&json).expect("Deserialization should succeed");

        assert_eq!(original, deserialized);
    }

    #[test]
    fn test_ollama_request_with_empty_content() {
        let request = OllamaChatRequest::new(
            "model".to_string(),
            "".to_string(),
            Some(OllamaOptions { temperature: 0.0 }),
        );
        let json = serde_json::to_string(&request).expect("Serialization should succeed");
        let deserialized: OllamaChatRequest =
            serde_json::from_str(&json).expect("Deserialization should succeed");

        assert_eq!(request, deserialized);
        assert_eq!(deserialized.messages[0].content, "");
    }

    #[test]
    fn test_ollama_request_with_unicode_content() {
        let unicode_content = "Hello 世界! 🌍 Café naïve résumé";
        let request = OllamaChatRequest::new(
            "model".to_string(),
            unicode_content.to_string(),
            Some(OllamaOptions { temperature: 0.0 }),
        );
        let json = serde_json::to_string(&request).expect("Serialization should succeed");
        let deserialized: OllamaChatRequest =
            serde_json::from_str(&json).expect("Deserialization should succeed");

        assert_eq!(request, deserialized);
        assert_eq!(deserialized.messages[0].content, unicode_content);
    }

    #[test]
    fn test_ollama_request_with_special_characters() {
        let special_content = r#"Line 1
Line 2 with "quotes" and 'apostrophes'
Tab:	End
Backslash: \ Forward slash: /
JSON special: {"key": "value"}"#;
        let request = OllamaChatRequest::new(
            "model".to_string(),
            special_content.to_string(),
            Some(OllamaOptions { temperature: 0.0 }),
        );
        let json = serde_json::to_string(&request).expect("Serialization should succeed");
        let deserialized: OllamaChatRequest =
            serde_json::from_str(&json).expect("Deserialization should succeed");

        assert_eq!(request, deserialized);
        assert_eq!(deserialized.messages[0].content, special_content);
    }

    #[test]
    fn test_ollama_request_deserialization_invalid_json() {
        let invalid_json = r#"{"model":"test","messages":[{"role":"user"],"stream":false}"#; // Missing content field
        let result: Result<OllamaChatRequest, _> = serde_json::from_str(invalid_json);
        assert!(result.is_err());
    }

    #[test]
    fn test_ollama_request_deserialization_missing_field() {
        let json_missing_model =
            r#"{"messages":[{"role":"user","content":"test"}],"stream":false}"#;
        let result: Result<OllamaChatRequest, _> = serde_json::from_str(json_missing_model);
        assert!(result.is_err());
    }

    #[test]
    fn test_ollama_request_with_empty_messages() {
        let request = OllamaChatRequest::with_messages(
            "model".to_string(),
            vec![],
            Some(OllamaOptions { temperature: 0.0 }),
        );
        let json = serde_json::to_string(&request).expect("Serialization should succeed");
        let deserialized: OllamaChatRequest =
            serde_json::from_str(&json).expect("Deserialization should succeed");

        assert_eq!(request, deserialized);
        assert_eq!(deserialized.messages.len(), 0);
        assert_eq!(deserialized.think, false);
    }

    #[test]
    fn test_ollama_request_with_think_true() {
        let mut request = OllamaChatRequest::new(
            "llama2".to_string(),
            "Test with thinking".to_string(),
            Some(OllamaOptions { temperature: 0.0 }),
        );
        request.think = true;

        let json = serde_json::to_string(&request).expect("Serialization should succeed");
        let expected_json = r#"{"model":"llama2","messages":[{"role":"user","content":"Test with thinking"}],"stream":false,"think":true,"options":{"temperature":0.0}}"#;
        assert_eq!(json, expected_json);

        let deserialized: OllamaChatRequest =
            serde_json::from_str(&json).expect("Deserialization should succeed");
        assert_eq!(deserialized.think, true);
    }

    #[test]
    fn test_ollama_request_deserialization_with_think_field() {
        let json = r#"{"model":"claude","messages":[{"role":"user","content":"Hello"}],"stream":false,"think":true}"#;
        let request: OllamaChatRequest =
            serde_json::from_str(json).expect("Deserialization should succeed");

        assert_eq!(request.model, "claude");
        assert_eq!(request.messages.len(), 1);
        assert_eq!(request.messages[0].role, "user");
        assert_eq!(request.messages[0].content, "Hello");
        assert_eq!(request.stream, false);
        assert_eq!(request.think, true);
    }
}
