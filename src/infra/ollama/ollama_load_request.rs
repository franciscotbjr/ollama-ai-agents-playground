use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct OllamaLoadRequest {
    pub name: String,
}

impl OllamaLoadRequest {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_ollama_load_request_new() {
        let request = OllamaLoadRequest::new("gemma3".to_string());
        assert_eq!(request.name, "gemma3");
    }

    #[test]
    fn test_ollama_load_request_serialization() {
        let request = OllamaLoadRequest::new("test-model".to_string());
        let serialized = serde_json::to_string(&request).expect("Serialization should succeed");

        let expected = r#"{"name":"test-model"}"#;
        assert_eq!(serialized, expected);
    }

    #[test]
    fn test_ollama_load_request_deserialization() {
        let json = r#"{"name":"deserialized-model"}"#;
        let request: OllamaLoadRequest =
            serde_json::from_str(json).expect("Deserialization should succeed");

        assert_eq!(request.name, "deserialized-model");
    }

    #[test]
    fn test_ollama_load_request_roundtrip_serialization() {
        let original = OllamaLoadRequest::new("roundtrip-model".to_string());
        let serialized = serde_json::to_string(&original).expect("Serialization should succeed");
        let deserialized: OllamaLoadRequest =
            serde_json::from_str(&serialized).expect("Deserialization should succeed");

        assert_eq!(original, deserialized);
    }

    #[test]
    fn test_ollama_load_request_clone() {
        let original = OllamaLoadRequest::new("clone-test".to_string());
        let cloned = original.clone();

        assert_eq!(original, cloned);
        assert_eq!(cloned.name, "clone-test");
    }

    #[test]
    fn test_ollama_load_request_partial_eq() {
        let request1 = OllamaLoadRequest::new("model1".to_string());
        let request2 = OllamaLoadRequest::new("model1".to_string());
        let request3 = OllamaLoadRequest::new("model2".to_string());

        assert_eq!(request1, request2);
        assert_ne!(request1, request3);
    }

    #[test]
    fn test_ollama_load_request_debug_format() {
        let request = OllamaLoadRequest::new("debug-model".to_string());
        let debug_string = format!("{:?}", request);

        assert!(debug_string.contains("OllamaLoadRequest"));
        assert!(debug_string.contains("name: \"debug-model\""));
    }

    #[test]
    fn test_ollama_load_request_with_empty_name() {
        let request = OllamaLoadRequest::new("".to_string());
        assert_eq!(request.name, "");
    }

    #[test]
    fn test_ollama_load_request_with_special_characters() {
        let model_name = "model-with_special.chars@123".to_string();
        let request = OllamaLoadRequest::new(model_name.clone());
        assert_eq!(request.name, model_name);
    }

    #[test]
    fn test_ollama_load_request_with_unicode() {
        let model_name = "模型-test-café".to_string();
        let request = OllamaLoadRequest::new(model_name.clone());
        assert_eq!(request.name, model_name);
    }
}
