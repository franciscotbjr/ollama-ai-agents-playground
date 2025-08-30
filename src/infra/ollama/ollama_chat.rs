use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct OllamaChat {
    pub role: String,
    pub content: String,
}

impl OllamaChat {
    pub fn new(role: String, content: String) -> Self {
        Self { role, content }
    }

    pub fn user(content: String) -> Self {
        Self::new("user".to_string(), content)
    }

    pub fn assistant(content: String) -> Self {
        Self::new("assistant".to_string(), content)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_ollama_message_new() {
        let message = OllamaChat::new(
            "system".to_string(),
            "You are a helpful assistant".to_string(),
        );

        assert_eq!(message.role, "system");
        assert_eq!(message.content, "You are a helpful assistant");
    }

    #[test]
    fn test_ollama_message_user() {
        let message = OllamaChat::user("What is the weather today?".to_string());

        assert_eq!(message.role, "user");
        assert_eq!(message.content, "What is the weather today?");
    }

    #[test]
    fn test_ollama_message_assistant() {
        let message = OllamaChat::assistant("I can help you with that!".to_string());

        assert_eq!(message.role, "assistant");
        assert_eq!(message.content, "I can help you with that!");
    }

    #[test]
    fn test_ollama_message_serialization() {
        let message = OllamaChat::user("Hello world".to_string());
        let json = serde_json::to_string(&message).expect("Serialization should succeed");

        let expected_json = r#"{"role":"user","content":"Hello world"}"#;
        assert_eq!(json, expected_json);
    }

    #[test]
    fn test_ollama_message_assistant_serialization() {
        let message = OllamaChat::assistant("Hello! How can I help you?".to_string());
        let json = serde_json::to_string(&message).expect("Serialization should succeed");

        let expected_json = r#"{"role":"assistant","content":"Hello! How can I help you?"}"#;
        assert_eq!(json, expected_json);
    }

    #[test]
    fn test_ollama_message_deserialization() {
        let json = r#"{"role":"user","content":"Test message"}"#;
        let message: OllamaChat =
            serde_json::from_str(json).expect("Deserialization should succeed");

        assert_eq!(message.role, "user");
        assert_eq!(message.content, "Test message");
    }

    #[test]
    fn test_ollama_message_deserialization_assistant() {
        let json = r#"{"role":"assistant","content":"I'm here to help!"}"#;
        let message: OllamaChat =
            serde_json::from_str(json).expect("Deserialization should succeed");

        assert_eq!(message.role, "assistant");
        assert_eq!(message.content, "I'm here to help!");
    }

    #[test]
    fn test_ollama_message_roundtrip_serialization() {
        let original = OllamaChat::new("custom".to_string(), "Custom role message".to_string());
        let json = serde_json::to_string(&original).expect("Serialization should succeed");
        let deserialized: OllamaChat =
            serde_json::from_str(&json).expect("Deserialization should succeed");

        assert_eq!(original, deserialized);
    }

    #[test]
    fn test_ollama_message_with_empty_content() {
        let message = OllamaChat::user("".to_string());
        let json = serde_json::to_string(&message).expect("Serialization should succeed");
        let deserialized: OllamaChat =
            serde_json::from_str(&json).expect("Deserialization should succeed");

        assert_eq!(message, deserialized);
        assert_eq!(deserialized.content, "");
        assert_eq!(deserialized.role, "user");
    }

    #[test]
    fn test_ollama_message_with_empty_role() {
        let message = OllamaChat::new("".to_string(), "Message with empty role".to_string());
        let json = serde_json::to_string(&message).expect("Serialization should succeed");
        let deserialized: OllamaChat =
            serde_json::from_str(&json).expect("Deserialization should succeed");

        assert_eq!(message, deserialized);
        assert_eq!(deserialized.role, "");
        assert_eq!(deserialized.content, "Message with empty role");
    }

    #[test]
    fn test_ollama_message_with_unicode_content() {
        let unicode_content = "Hello 世界! 🌍 Café naïve résumé";
        let message = OllamaChat::user(unicode_content.to_string());
        let json = serde_json::to_string(&message).expect("Serialization should succeed");
        let deserialized: OllamaChat =
            serde_json::from_str(&json).expect("Deserialization should succeed");

        assert_eq!(message, deserialized);
        assert_eq!(deserialized.content, unicode_content);
    }

    #[test]
    fn test_ollama_message_with_special_characters() {
        let special_content = r#"Line 1
Line 2 with "quotes" and 'apostrophes'
Tab:	End
Backslash: \ Forward slash: /
JSON special: {"key": "value"}"#;
        let message = OllamaChat::assistant(special_content.to_string());
        let json = serde_json::to_string(&message).expect("Serialization should succeed");
        let deserialized: OllamaChat =
            serde_json::from_str(&json).expect("Deserialization should succeed");

        assert_eq!(message, deserialized);
        assert_eq!(deserialized.content, special_content);
    }

    #[test]
    fn test_ollama_message_long_content() {
        let long_content = "This is a very long message that contains many words and should test the serialization and deserialization of longer content. ".repeat(100);
        let message = OllamaChat::user(long_content.clone());
        let json = serde_json::to_string(&message).expect("Serialization should succeed");
        let deserialized: OllamaChat =
            serde_json::from_str(&json).expect("Deserialization should succeed");

        assert_eq!(message, deserialized);
        assert_eq!(deserialized.content, long_content);
    }

    #[test]
    fn test_ollama_message_custom_roles() {
        let roles = vec!["system", "function", "tool", "custom_role"];

        for role in roles {
            let message = OllamaChat::new(role.to_string(), format!("Message for {}", role));
            let json = serde_json::to_string(&message).expect("Serialization should succeed");
            let deserialized: OllamaChat =
                serde_json::from_str(&json).expect("Deserialization should succeed");

            assert_eq!(message, deserialized);
            assert_eq!(deserialized.role, role);
        }
    }

    #[test]
    fn test_ollama_message_clone() {
        let original = OllamaChat::user("Original message".to_string());
        let cloned = original.clone();

        assert_eq!(original, cloned);
        assert_eq!(original.role, cloned.role);
        assert_eq!(original.content, cloned.content);

        // Verify they are separate instances
        assert_ne!(&original as *const OllamaChat, &cloned as *const OllamaChat);
    }

    #[test]
    fn test_ollama_message_partial_eq() {
        let message1 = OllamaChat::user("Same content".to_string());
        let message2 = OllamaChat::user("Same content".to_string());
        let message3 = OllamaChat::user("Different content".to_string());
        let message4 = OllamaChat::assistant("Same content".to_string());

        assert_eq!(message1, message2);
        assert_ne!(message1, message3);
        assert_ne!(message1, message4);
    }

    #[test]
    fn test_ollama_message_debug_format() {
        let message = OllamaChat::user("Debug test message".to_string());
        let debug_string = format!("{:?}", message);

        assert!(debug_string.contains("OllamaChat"));
        assert!(debug_string.contains("user"));
        assert!(debug_string.contains("Debug test message"));
    }

    #[test]
    fn test_ollama_message_deserialization_invalid_json() {
        let invalid_json = r#"{"role":"user","invalid_field":"value"}"#; // Missing content field
        let result: Result<OllamaChat, _> = serde_json::from_str(invalid_json);
        assert!(result.is_err());
    }

    #[test]
    fn test_ollama_message_deserialization_missing_role() {
        let json_missing_role = r#"{"content":"Message without role"}"#;
        let result: Result<OllamaChat, _> = serde_json::from_str(json_missing_role);
        assert!(result.is_err());
    }

    #[test]
    fn test_ollama_message_deserialization_extra_fields() {
        let json_extra_fields =
            r#"{"role":"user","content":"Message with extra fields","extra":"ignored"}"#;
        let result: Result<OllamaChat, _> = serde_json::from_str(json_extra_fields);
        assert!(result.is_ok());

        let message = result.unwrap();
        assert_eq!(message.role, "user");
        assert_eq!(message.content, "Message with extra fields");
    }

    #[test]
    fn test_ollama_message_constructors_consistency() {
        let user_content = "Test content";
        let assistant_content = "Assistant response";

        let user_msg1 = OllamaChat::user(user_content.to_string());
        let user_msg2 = OllamaChat::new("user".to_string(), user_content.to_string());

        let assistant_msg1 = OllamaChat::assistant(assistant_content.to_string());
        let assistant_msg2 =
            OllamaChat::new("assistant".to_string(), assistant_content.to_string());

        assert_eq!(user_msg1, user_msg2);
        assert_eq!(assistant_msg1, assistant_msg2);
    }
}
