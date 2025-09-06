use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct OllamaCheckRequest {
    pub model: String,
}

impl OllamaCheckRequest {
    pub fn new(model: String) -> Self {
        Self { model }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_ollama_check_request_new() {
        let request = OllamaCheckRequest::new("gemma3".to_string());
        assert_eq!(request.model, "gemma3");
    }

    #[test]
    fn test_ollama_check_request_serialization() {
        let request = OllamaCheckRequest::new("test-model".to_string());
        let serialized = serde_json::to_string(&request).expect("Serialization should succeed");

        let expected = r#"{"model":"test-model"}"#;
        assert_eq!(serialized, expected);
    }

    #[test]
    fn test_ollama_check_request_deserialization() {
        let json = r#"{"model":"deserialized-model"}"#;
        let request: OllamaCheckRequest =
            serde_json::from_str(json).expect("Deserialization should succeed");

        assert_eq!(request.model, "deserialized-model");
    }

    #[test]
    fn test_ollama_check_request_roundtrip_serialization() {
        let original = OllamaCheckRequest::new("roundtrip-model".to_string());
        let serialized = serde_json::to_string(&original).expect("Serialization should succeed");
        let deserialized: OllamaCheckRequest =
            serde_json::from_str(&serialized).expect("Deserialization should succeed");

        assert_eq!(original, deserialized);
    }

    #[test]
    fn test_ollama_check_request_clone() {
        let original = OllamaCheckRequest::new("clone-test".to_string());
        let cloned = original.clone();

        assert_eq!(original, cloned);
        assert_eq!(cloned.model, "clone-test");
    }

    #[test]
    fn test_ollama_check_request_partial_eq() {
        let request1 = OllamaCheckRequest::new("model1".to_string());
        let request2 = OllamaCheckRequest::new("model1".to_string());
        let request3 = OllamaCheckRequest::new("model2".to_string());

        assert_eq!(request1, request2);
        assert_ne!(request1, request3);
    }

    #[test]
    fn test_ollama_check_request_debug_format() {
        let request = OllamaCheckRequest::new("debug-model".to_string());
        let debug_string = format!("{:?}", request);

        assert!(debug_string.contains("OllamaCheckRequest"));
        assert!(debug_string.contains("model: \"debug-model\""));
    }

    #[test]
    fn test_ollama_check_request_with_empty_name() {
        let request = OllamaCheckRequest::new("".to_string());
        assert_eq!(request.model, "");
    }

    #[test]
    fn test_ollama_check_request_with_special_characters() {
        let model_name = "model-with_special.chars@123".to_string();
        let request = OllamaCheckRequest::new(model_name.clone());
        assert_eq!(request.model, model_name);
    }

    #[test]
    fn test_ollama_check_request_with_unicode() {
        let model_name = "模型-test-café".to_string();
        let request = OllamaCheckRequest::new(model_name.clone());
        assert_eq!(request.model, model_name);
    }
}
