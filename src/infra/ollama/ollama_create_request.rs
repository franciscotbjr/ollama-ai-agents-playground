use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct OllamaCreateRequest {
    pub from: String,
    pub system: String,
    pub name: String,
}

impl OllamaCreateRequest {
    pub fn new(from: String, system: String, name: String) -> Self {
        Self { from, system, name }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_ollama_create_request_new() {
        let request = OllamaCreateRequest::new(
            "llama2".to_string(),
            "You are a helpful assistant".to_string(),
            "my_assistant".to_string(),
        );
        assert_eq!(request.from, "llama2");
        assert_eq!(request.system, "You are a helpful assistant");
        assert_eq!(request.name, "my_assistant");
    }

    #[test]
    fn test_ollama_create_request_serialization() {
        let request = OllamaCreateRequest::new(
            "llama2".to_string(),
            "Test system message".to_string(),
            "test_assistant".to_string(),
        );
        let json = serde_json::to_string(&request).expect("Serialization should succeed");

        let expected_json =
            r#"{"from":"llama2","system":"Test system message","name":"test_assistant"}"#;
        assert_eq!(json, expected_json);
    }

    #[test]
    fn test_ollama_create_request_deserialization() {
        let json =
            r#"{"from":"foundation","system":"You are an AI assistant","name":"claude_assistant"}"#;
        let request: OllamaCreateRequest =
            serde_json::from_str(json).expect("Deserialization should succeed");
        assert_eq!(request.from, "foundation");
        assert_eq!(request.system, "You are an AI assistant");
        assert_eq!(request.name, "claude_assistant");
    }

    #[test]
    fn test_ollama_create_request_roundtrip_serialization() {
        let original = OllamaCreateRequest::new(
            "test-base".to_string(),
            "Test system prompt".to_string(),
            "test-model".to_string(),
        );
        let json = serde_json::to_string(&original).expect("Serialization should succeed");
        let deserialized: OllamaCreateRequest =
            serde_json::from_str(&json).expect("Deserialization should succeed");

        assert_eq!(original, deserialized);
    }

    #[test]
    fn test_ollama_create_request_partial_eq() {
        let request1 = OllamaCreateRequest::new(
            "same-base".to_string(),
            "Same system".to_string(),
            "same-model".to_string(),
        );
        let request2 = OllamaCreateRequest::new(
            "same-base".to_string(),
            "Same system".to_string(),
            "same-model".to_string(),
        );
        let request3 = OllamaCreateRequest::new(
            "different-base".to_string(),
            "Different system".to_string(),
            "different-model".to_string(),
        );

        assert_eq!(request1, request2);
        assert_ne!(request1, request3);
    }

    #[test]
    fn test_ollama_create_request_debug_format() {
        let request = OllamaCreateRequest::new(
            "debug-base".to_string(),
            "Debug system message".to_string(),
            "debug-model".to_string(),
        );
        let debug_string = format!("{:?}", request);

        assert!(debug_string.contains("OllamaCreateRequest"));
        assert!(debug_string.contains("debug-base"));
        assert!(debug_string.contains("Debug system message"));
    }

    #[test]
    fn test_ollama_create_request_deserialization_missing_field() {
        let json_missing_name = r#"{"from":"base","system":"prompt"}"#;
        let result: Result<OllamaCreateRequest, _> = serde_json::from_str(json_missing_name);
        assert!(result.is_err());
    }

    #[test]
    fn test_ollama_create_request_with_unicode_content() {
        let unicode_system = "You are a helpful assistant. 你好世界! 🤖 Café naïve";
        let request = OllamaCreateRequest::new(
            "base".to_string(),
            unicode_system.to_string(),
            "unicode-model".to_string(),
        );
        let json = serde_json::to_string(&request).expect("Serialization should succeed");
        let deserialized: OllamaCreateRequest =
            serde_json::from_str(&json).expect("Deserialization should succeed");

        assert_eq!(deserialized.system, unicode_system);
    }

    #[test]
    fn test_ollama_create_request_with_special_characters() {
        let special_system = r#"System with "quotes", 'apostrophes', and \backslashes\, plus newlines:
Line 1
Line 2"#;
        let request = OllamaCreateRequest::new(
            "base".to_string(),
            special_system.to_string(),
            "special-model".to_string(),
        );
        let json = serde_json::to_string(&request).expect("Serialization should succeed");
        let deserialized: OllamaCreateRequest =
            serde_json::from_str(&json).expect("Deserialization should succeed");

        assert_eq!(deserialized.system, special_system);
    }

    #[test]
    fn test_ollama_create_request_with_long_system_message() {
        let long_system = "This is a very long system message that contains many words and should test the serialization and deserialization of longer content. ".repeat(100);
        let request = OllamaCreateRequest::new(
            "base".to_string(),
            long_system.clone(),
            "long-model".to_string(),
        );
        let json = serde_json::to_string(&request).expect("Serialization should succeed");
        let deserialized: OllamaCreateRequest =
            serde_json::from_str(&json).expect("Deserialization should succeed");

        assert_eq!(deserialized.system, long_system);
    }

    #[test]
    fn test_ollama_create_request_realistic_model_names() {
        let models = [
            ("llama3.1:8b", "meta-llama/llama-3.1-8b"),
            ("qwen2.5:14b", "alibaba-cloud/qwen-2.5-14b"),
            ("mistral:7b", "mistralai/mistral-7b"),
            ("codellama:13b", "meta-llama/codellama-13b"),
        ];

        for (model_name, base_model) in models.iter() {
            let request = OllamaCreateRequest::new(
                base_model.to_string(),
                "You are a helpful AI assistant".to_string(),
                model_name.to_string(),
            );

            assert_eq!(request.from, *base_model);
            assert_eq!(request.name, *model_name);

            let json = serde_json::to_string(&request).expect("Serialization should succeed");
            let deserialized: OllamaCreateRequest =
                serde_json::from_str(&json).expect("Deserialization should succeed");

            assert_eq!(request, deserialized);
        }
    }

    #[test]
    fn test_ollama_create_request_consistency() {
        let models = vec![
            "personal-assistant-pro",
            "senior-code-reviewer",
            "technical-documentation-expert",
            "senior-financial-analyst",
        ];

        for model_name in models {
            let request = OllamaCreateRequest::new(
                "llama3.1:8b".to_string(),
                "Professional AI assistant".to_string(),
                model_name.to_string(),
            );

            assert_eq!(request.from, "llama3.1:8b");
            assert_eq!(request.name, model_name);
        }
    }

    // Concrete test cases based on real Ollama create operations
    #[test]
    fn test_ollama_create_request_with_layer_operations() {
        let request = OllamaCreateRequest::new(
            "llama3.1:8b".to_string(),
            "You are a specialized AI assistant with layer-based processing".to_string(),
            "custom-assistant".to_string(),
        );

        // Test that it serializes correctly for Ollama API
        let json = serde_json::to_string(&request).expect("Serialization should succeed");
        assert!(json.contains("custom-assistant"));
        assert!(json.contains("llama3.1:8b"));
        assert!(json.contains("layer-based processing"));
    }

    #[test]
    fn test_ollama_create_request_with_sha256_layer_references() {
        let system_with_layers = r#"You are an AI assistant. Using layers:
- Base layer: sha256:7f4030143c1c477224c5434f8272c662a8b042079a0a584f0a27a1684fe2e1fa
- Custom layer: sha256:ae370d884f108d16e7cc8fd5259ebc5773a0afa6e078b11f4ed7e39a27e0dfc4"#;

        let request = OllamaCreateRequest::new(
            "base-model".to_string(),
            system_with_layers.to_string(),
            "layered-assistant".to_string(),
        );

        let json = serde_json::to_string(&request).expect("Serialization should succeed");
        assert!(json.contains("sha256:7f4030143c1c"));
        assert!(json.contains("sha256:ae370d884f10"));
    }

    #[test]
    fn test_ollama_create_request_layer_creation_simulation() {
        // Test creating multiple models with layer references
        let models = vec![
            ("model-v1", "using existing layer sha256:abc123"),
            ("model-v2", "creating new layer sha256:def456"),
            ("model-v3", "writing manifest"),
        ];

        for (model_name, layer_info) in models {
            let request = OllamaCreateRequest::new(
                "foundation".to_string(),
                format!("AI assistant with {}", layer_info),
                model_name.to_string(),
            );

            assert!(request.system.contains(layer_info));
        }
    }

    #[test]
    fn test_ollama_create_request_status_response_compatibility() {
        // Test compatibility with OllamaCreateResponse status messages
        let request = OllamaCreateRequest::new(
            "base-model".to_string(),
            "Status-aware AI assistant".to_string(),
            "status-test".to_string(),
        );

        // Verify serialization format matches expected status response format
        let json = serde_json::to_string(&request).expect("Serialization should succeed");
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();

        assert!(parsed["from"].is_string());
        assert!(parsed["system"].is_string());
        assert!(parsed["name"].is_string());
    }
}
