use serde::{Deserialize, Serialize};

use crate::role::Role;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct OllamaChat {
    pub role: Option<Role>,
    pub content: String,
}

impl OllamaChat {
    pub fn new(role: Option<Role>, content: String) -> Self {
        Self { role, content }
    }

    pub fn user(content: String) -> Self {
        Self::new(Some(Role::User), content)
    }

    pub fn assistant(content: String) -> Self {
        Self::new(Some(Role::Assistant), content)
    }

    pub fn system(content: String) -> Self {
        Self::new(Some(Role::System), content)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_ollama_chat_new_with_user_role() {
        let chat = OllamaChat::new(Some(Role::User), "Hello world".to_string());
        assert_eq!(chat.role, Some(Role::User));
        assert_eq!(chat.content, "Hello world");
    }

    #[test]
    fn test_ollama_chat_new_with_assistant_role() {
        let chat = OllamaChat::new(Some(Role::Assistant), "Hi there!".to_string());
        assert_eq!(chat.role, Some(Role::Assistant));
        assert_eq!(chat.content, "Hi there!");
    }

    #[test]
    fn test_ollama_chat_new_with_system_role() {
        let chat = OllamaChat::new(Some(Role::System), "System prompt".to_string());
        assert_eq!(chat.role, Some(Role::System));
        assert_eq!(chat.content, "System prompt");
    }

    #[test]
    fn test_ollama_chat_new_with_no_role() {
        let chat = OllamaChat::new(None, "No role message".to_string());
        assert_eq!(chat.role, None);
        assert_eq!(chat.content, "No role message");
    }

    #[test]
    fn test_ollama_chat_user_constructor() {
        let chat = OllamaChat::user("User message".to_string());
        assert_eq!(chat.role, Some(Role::User));
        assert_eq!(chat.content, "User message");
    }

    #[test]
    fn test_ollama_chat_assistant_constructor() {
        let chat = OllamaChat::assistant("Assistant response".to_string());
        assert_eq!(chat.role, Some(Role::Assistant));
        assert_eq!(chat.content, "Assistant response");
    }

    #[test]
    fn test_ollama_chat_system_constructor() {
        let chat = OllamaChat::system("System instruction".to_string());
        assert_eq!(chat.role, Some(Role::System));
        assert_eq!(chat.content, "System instruction");
    }

    #[test]
    fn test_ollama_chat_with_empty_content() {
        let chat = OllamaChat::user("".to_string());
        assert_eq!(chat.role, Some(Role::User));
        assert_eq!(chat.content, "");
    }

    #[test]
    fn test_ollama_chat_with_multiline_content() {
        let multiline_content = "Line 1\nLine 2\nLine 3".to_string();
        let chat = OllamaChat::assistant(multiline_content.clone());
        assert_eq!(chat.role, Some(Role::Assistant));
        assert_eq!(chat.content, multiline_content);
    }

    #[test]
    fn test_ollama_chat_with_unicode_content() {
        let unicode_content = "Hello 世界! 🌍 Café naïve résumé".to_string();
        let chat = OllamaChat::user(unicode_content.clone());
        assert_eq!(chat.role, Some(Role::User));
        assert_eq!(chat.content, unicode_content);
    }

    #[test]
    fn test_ollama_chat_with_special_characters() {
        let special_content = r#"{"key": "value", "quotes": "\"test\""}"#.to_string();
        let chat = OllamaChat::system(special_content.clone());
        assert_eq!(chat.role, Some(Role::System));
        assert_eq!(chat.content, special_content);
    }

    #[test]
    fn test_ollama_chat_clone() {
        let original = OllamaChat::user("Original message".to_string());
        let cloned = original.clone();
        
        assert_eq!(original, cloned);
        assert_eq!(original.role, cloned.role);
        assert_eq!(original.content, cloned.content);
    }

    #[test]
    fn test_ollama_chat_partial_eq() {
        let chat1 = OllamaChat::user("Same message".to_string());
        let chat2 = OllamaChat::user("Same message".to_string());
        let chat3 = OllamaChat::user("Different message".to_string());
        let chat4 = OllamaChat::assistant("Same message".to_string());

        assert_eq!(chat1, chat2);
        assert_ne!(chat1, chat3);
        assert_ne!(chat1, chat4);
    }

    #[test]
    fn test_ollama_chat_serialization() {
        let chat = OllamaChat::user("Test message".to_string());
        let json = serde_json::to_string(&chat).expect("Serialization should succeed");
        
        let expected_json = r#"{"role":"user","content":"Test message"}"#;
        assert_eq!(json, expected_json);
    }

    #[test]
    fn test_ollama_chat_serialization_with_none_role() {
        let chat = OllamaChat::new(None, "No role message".to_string());
        let json = serde_json::to_string(&chat).expect("Serialization should succeed");
        
        let expected_json = r#"{"role":null,"content":"No role message"}"#;
        assert_eq!(json, expected_json);
    }

    #[test]
    fn test_ollama_chat_serialization_all_roles() {
        let user_chat = OllamaChat::user("User msg".to_string());
        let assistant_chat = OllamaChat::assistant("Assistant msg".to_string());
        let system_chat = OllamaChat::system("System msg".to_string());

        let user_json = serde_json::to_string(&user_chat).unwrap();
        let assistant_json = serde_json::to_string(&assistant_chat).unwrap();
        let system_json = serde_json::to_string(&system_chat).unwrap();

        assert_eq!(user_json, r#"{"role":"user","content":"User msg"}"#);
        assert_eq!(assistant_json, r#"{"role":"assistant","content":"Assistant msg"}"#);
        assert_eq!(system_json, r#"{"role":"system","content":"System msg"}"#);
    }

    #[test]
    fn test_ollama_chat_deserialization() {
        let json = r#"{"role":"user","content":"Test message"}"#;
        let chat: OllamaChat = serde_json::from_str(json).expect("Deserialization should succeed");
        
        assert_eq!(chat.role, Some(Role::User));
        assert_eq!(chat.content, "Test message");
    }

    #[test]
    fn test_ollama_chat_deserialization_with_null_role() {
        let json = r#"{"role":null,"content":"No role message"}"#;
        let chat: OllamaChat = serde_json::from_str(json).expect("Deserialization should succeed");
        
        assert_eq!(chat.role, None);
        assert_eq!(chat.content, "No role message");
    }

    #[test]
    fn test_ollama_chat_deserialization_all_roles() {
        let user_json = r#"{"role":"user","content":"User message"}"#;
        let assistant_json = r#"{"role":"assistant","content":"Assistant message"}"#;
        let system_json = r#"{"role":"system","content":"System message"}"#;

        let user_chat: OllamaChat = serde_json::from_str(user_json).unwrap();
        let assistant_chat: OllamaChat = serde_json::from_str(assistant_json).unwrap();
        let system_chat: OllamaChat = serde_json::from_str(system_json).unwrap();

        assert_eq!(user_chat.role, Some(Role::User));
        assert_eq!(assistant_chat.role, Some(Role::Assistant));
        assert_eq!(system_chat.role, Some(Role::System));
    }

    #[test]
    fn test_ollama_chat_roundtrip_serialization() {
        let original = OllamaChat::assistant("Roundtrip test".to_string());
        let json = serde_json::to_string(&original).expect("Serialization should succeed");
        let deserialized: OllamaChat = serde_json::from_str(&json).expect("Deserialization should succeed");
        
        assert_eq!(original, deserialized);
    }

    #[test]
    fn test_ollama_chat_deserialization_invalid_json() {
        let invalid_json = r#"{"role":"user","invalid_field":"test"}"#; // Missing content field
        let result: Result<OllamaChat, _> = serde_json::from_str(invalid_json);
        assert!(result.is_err());
    }

    #[test]
    fn test_ollama_chat_deserialization_invalid_role() {
        let invalid_role_json = r#"{"role":"invalid_role","content":"test"}"#;
        let result: Result<OllamaChat, _> = serde_json::from_str(invalid_role_json);
        assert!(result.is_err());
    }

    #[test]
    fn test_ollama_chat_with_very_long_content() {
        let long_content = "a".repeat(10000);
        let chat = OllamaChat::user(long_content.clone());
        assert_eq!(chat.content, long_content);
        assert_eq!(chat.role, Some(Role::User));
    }

    #[test]
    fn test_ollama_chat_debug_format() {
        let chat = OllamaChat::user("Debug test".to_string());
        let debug_string = format!("{:?}", chat);
        assert!(debug_string.contains("OllamaChat"));
        assert!(debug_string.contains("User"));
        assert!(debug_string.contains("Debug test"));
    }

    #[test]
    fn test_ollama_chat_with_json_content() {
        let json_content = r#"{"intent":"send_email","params":{"recipient":"test","message":"hello"}}"#.to_string();
        let chat = OllamaChat::assistant(json_content.clone());
        assert_eq!(chat.content, json_content);
        assert_eq!(chat.role, Some(Role::Assistant));
    }

    #[test]
    fn test_ollama_chat_serialization_with_escaped_characters() {
        let content_with_escapes = "Line 1\nLine 2\tTabbed\r\nWindows line ending".to_string();
        let chat = OllamaChat::system(content_with_escapes.clone());
        let json = serde_json::to_string(&chat).expect("Serialization should succeed");
        let deserialized: OllamaChat = serde_json::from_str(&json).expect("Deserialization should succeed");
        
        assert_eq!(chat, deserialized);
        assert_eq!(deserialized.content, content_with_escapes);
    }
}
