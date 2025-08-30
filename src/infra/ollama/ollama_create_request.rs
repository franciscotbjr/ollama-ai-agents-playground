use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct OllamaCreateRequest {
    pub from: String,
    pub system: String,
    pub name: String,
}

impl OllamaCreateRequest {
    pub fn new(from: String, system: String, name: String) -> Self {
        Self {
            from,
            system,
            name,
        }
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
            "base_model".to_string(),
            "You are a helpful assistant".to_string(),
        );

        assert_eq!(request.system, "You are a helpful assistant");
        assert_eq!(request.name, "my_assistant");
    }

    #[test]
    fn test_ollama_create_request_serialization() {
        let request = OllamaCreateRequest::new(
            "llama2".to_string(),
            "base_model".to_string(),
            "Test system message".to_string(),
        );
        let json = serde_json::to_string(&request).expect("Serialization should succeed");

        let expected_json = r#"{"model":"llama2","from":"base_model","system":"Test system message","name":"test_assistant"}"#;
        assert_eq!(json, expected_json);
    }

    #[test]
    fn test_ollama_create_request_deserialization() {
        let json = r#"{"model":"claude","from":"foundation","system":"You are an AI assistant","name":"claude_assistant"}"#;
        let request: OllamaCreateRequest =
            serde_json::from_str(json).expect("Deserialization should succeed");

        assert_eq!(request.from, "foundation");
        assert_eq!(request.system, "You are an AI assistant");
        assert_eq!(request.name, "claude_assistant");
    }

    #[test]
    fn test_ollama_create_request_roundtrip_serialization() {
        let original = OllamaCreateRequest::new(
            "test-model".to_string(),
            "test-base".to_string(),
            "Test system prompt".to_string(),
        );
        let json = serde_json::to_string(&original).expect("Serialization should succeed");
        let deserialized: OllamaCreateRequest =
            serde_json::from_str(&json).expect("Deserialization should succeed");

        assert_eq!(original, deserialized);
    }

    #[test]
    fn test_ollama_create_request_with_empty_strings() {
        let request = OllamaCreateRequest::new(
            "".to_string(),
            "".to_string(),
            "".to_string(),
        );
        let json = serde_json::to_string(&request).expect("Serialization should succeed");
        let deserialized: OllamaCreateRequest =
            serde_json::from_str(&json).expect("Deserialization should succeed");

        assert_eq!(request, deserialized);
        assert_eq!(deserialized.from, "");
        assert_eq!(deserialized.system, "");
        assert_eq!(deserialized.name, "");
    }

    #[test]
    fn test_ollama_create_request_with_unicode_content() {
        let unicode_system = "Hello 世界! 🌍 Café naïve résumé";
        let unicode_name = "助手_🤖";
        let request = OllamaCreateRequest::new(
            "模型".to_string(),
            "基础模型".to_string(),
            unicode_system.to_string(),
        );
        let json = serde_json::to_string(&request).expect("Serialization should succeed");
        let deserialized: OllamaCreateRequest =
            serde_json::from_str(&json).expect("Deserialization should succeed");

        assert_eq!(request, deserialized);
        assert_eq!(deserialized.from, "基础模型");
        assert_eq!(deserialized.system, unicode_system);
        assert_eq!(deserialized.name, unicode_name);
    }

    #[test]
    fn test_ollama_create_request_with_special_characters() {
        let special_system = r#"Line 1
Line 2 with "quotes" and 'apostrophes'
Tab:	End
Backslash: \ Forward slash: /
JSON special: {"key": "value"}"#;
        let request = OllamaCreateRequest::new(
            "model-v1.0".to_string(),
            "base-model_v2".to_string(),
            special_system.to_string(),
        );
        let json = serde_json::to_string(&request).expect("Serialization should succeed");
        let deserialized: OllamaCreateRequest =
            serde_json::from_str(&json).expect("Deserialization should succeed");

        assert_eq!(request, deserialized);
        assert_eq!(deserialized.system, special_system);
    }

    #[test]
    fn test_ollama_create_request_deserialization_invalid_json() {
        let invalid_json = r#"{"model":"test","from":"base","system":"test"}"#; // Missing name field
        let result: Result<OllamaCreateRequest, _> = serde_json::from_str(invalid_json);
        assert!(result.is_err());
    }

    #[test]
    fn test_ollama_create_request_deserialization_missing_field() {
        let json_missing_model = r#"{"from":"base","system":"You are helpful","name":"assistant"}"#;
        let result: Result<OllamaCreateRequest, _> = serde_json::from_str(json_missing_model);
        assert!(result.is_err());
    }

    #[test]
    fn test_ollama_create_request_with_long_system_message() {
        let long_system = "This is a very long system message that contains many words and should test the serialization and deserialization of longer content. ".repeat(100);
        let request = OllamaCreateRequest::new(
            "large-model".to_string(),
            "foundation".to_string(),
            long_system.clone(),
        );
        let json = serde_json::to_string(&request).expect("Serialization should succeed");
        let deserialized: OllamaCreateRequest =
            serde_json::from_str(&json).expect("Deserialization should succeed");

        assert_eq!(request, deserialized);
        assert_eq!(deserialized.system, long_system);
    }

    #[test]
    fn test_ollama_create_request_clone() {
        let original = OllamaCreateRequest::new(
            "original-model".to_string(),
            "base".to_string(),
            "Original system message".to_string(),
        );
        let cloned = original.clone();

        assert_eq!(original, cloned);
        assert_eq!(original.from, cloned.from);
        assert_eq!(original.system, cloned.system);
        assert_eq!(original.name, cloned.name);

        // Verify they are separate instances
        assert_ne!(
            &original as *const OllamaCreateRequest,
            &cloned as *const OllamaCreateRequest
        );
    }

    #[test]
    fn test_ollama_create_request_partial_eq() {
        let request1 = OllamaCreateRequest::new(
            "model".to_string(),
            "base".to_string(),
            "Same system".to_string(),
        );
        let request2 = OllamaCreateRequest::new(
            "model".to_string(),
            "base".to_string(),
            "Same system".to_string(),
        );
        let request3 = OllamaCreateRequest::new(
            "different".to_string(),
            "base".to_string(),
            "Same system".to_string(),
        );
        let request4 = OllamaCreateRequest::new(
            "model".to_string(),
            "different".to_string(),
            "Same system".to_string(),
        );
        let request5 = OllamaCreateRequest::new(
            "model".to_string(),
            "base".to_string(),
            "Different system".to_string(),
        );
        let request6 = OllamaCreateRequest::new(
            "model".to_string(),
            "base".to_string(),
            "Same system".to_string(),
        );

        assert_eq!(request1, request2);
        assert_ne!(request1, request3);
        assert_ne!(request1, request4);
        assert_ne!(request1, request5);
        assert_ne!(request1, request6);
    }

    #[test]
    fn test_ollama_create_request_debug_format() {
        let request = OllamaCreateRequest::new(
            "debug-model".to_string(),
            "debug-base".to_string(),
            "Debug test message".to_string(),
        );
        let debug_string = format!("{:?}", request);

        assert!(debug_string.contains("OllamaCreateRequest"));
        assert!(debug_string.contains("debug-model"));
        assert!(debug_string.contains("debug-base"));
        assert!(debug_string.contains("Debug test message"));
        assert!(debug_string.contains("debug_assistant"));
    }

    #[test]
    fn test_ollama_create_request_deserialization_extra_fields() {
        let json_extra_fields = r#"{"model":"test","from":"base","system":"Test system","name":"test_assistant","extra":"ignored"}"#;
        let result: Result<OllamaCreateRequest, _> = serde_json::from_str(json_extra_fields);
        assert!(result.is_ok());

        let request = result.unwrap();
        assert_eq!(request.from, "base");
        assert_eq!(request.system, "Test system");
        assert_eq!(request.name, "test_assistant");
    }

    #[test]
    fn test_ollama_create_request_consistency() {
        let model = "consistent-model";
        let from = "consistent-base";
        let system = "Consistent system message";
        let name = "consistent_assistant";

        let request = OllamaCreateRequest::new(
            model.to_string(),
            from.to_string(),
            system.to_string(),
        );

        assert_eq!(request.from, from);
        assert_eq!(request.system, system);
        assert_eq!(request.name, name);
    }

    // Concrete test cases based on real-world Ollama create request example
    #[test]
    fn test_ollama_create_request_with_layer_operations() {
        let request = OllamaCreateRequest::new(
            "llama2:7b".to_string(),
            "llama2".to_string(),
            "You are a helpful AI assistant specialized in code analysis.".to_string(),
        );

        // Test that request can be serialized for layer operations
        let json = serde_json::to_string(&request).expect("Serialization should succeed");
        assert!(json.contains("llama2:7b"));
        assert!(json.contains("code_assistant"));

        // Simulate the creation process with status responses
        let status_responses = vec![
            "using existing layer sha256:7f4030143c1c477224c5434f8272c662a8b042079a0a584f0a27a1684fe2e1fa",
            "using existing layer sha256:ae370d884f108d16e7cc8fd5259ebc5773a0afa6e078b11f4ed7e39a27e0dfc4",
            "creating new layer sha256:941b69ca7dc2a85c053c38d9e8029c9df6224e545060954fa97587f87c044a64",
            "writing manifest",
            "success",
        ];

        // Verify that status responses would be valid
        for status in &status_responses {
            assert!(!status.is_empty());
            assert!(status.len() > 5);
        }
    }

    #[test]
    fn test_ollama_create_request_realistic_model_names() {
        let test_cases = vec![
            ("llama2:7b", "llama2", "personal_assistant"),
            ("codellama:13b", "codellama", "code_helper"),
            ("mistral:7b-instruct", "mistral", "chat_bot"),
            ("gemma:2b", "gemma", "mini_assistant"),
            ("phi3:3.8b", "phi3", "reasoning_agent"),
        ];

        for (model, from, name) in test_cases {
            let request = OllamaCreateRequest::new(
                model.to_string(),
                from.to_string(),
                "You are a specialized AI assistant.".to_string(),
            );

            assert_eq!(request.from, from);
            assert_eq!(request.name, name);

            // Ensure it can be serialized
            let json = serde_json::to_string(&request).expect("Serialization should succeed");
            assert!(json.contains(from));
            assert!(json.contains(name));
        }
    }

    #[test]
    fn test_ollama_create_request_layer_creation_simulation() {
        let request = OllamaCreateRequest::new(
            "custom-model:latest".to_string(),
            "base-model:7b".to_string(),
            "You are an expert in data analysis and visualization.".to_string(),
        );

        // Test serialization for layer creation process
        let json = serde_json::to_string(&request).expect("Serialization should succeed");

        // Verify the JSON structure matches what Ollama expects
        assert!(json.contains("\"model\":\"custom-model:latest\""));
        assert!(json.contains("\"from\":\"base-model:7b\""));
        assert!(json.contains("\"name\":\"data_analyst\""));
        assert!(json.contains("\"system\":\"You are an expert in data analysis"));

        // Simulate response parsing (what would come back from Ollama)
        let expected_status_sequence = vec![
            "using existing layer",
            "creating new layer",
            "writing manifest",
            "success",
        ];

        // Verify we can work with these status types
        for status_type in &expected_status_sequence {
            assert!(status_type.len() > 0);
        }
    }

    #[test]
    fn test_ollama_create_request_with_sha256_layer_references() {
        let request = OllamaCreateRequest::new(
            "specialized-model".to_string(),
            "foundation-model".to_string(),
            "You are a domain-specific assistant.".to_string(),
        );

        // Test that the request works with layer hash references
        let layer_hashes = vec![
            "sha256:7f4030143c1c477224c5434f8272c662a8b042079a0a584f0a27a1684fe2e1fa",
            "sha256:ae370d884f108d16e7cc8fd5259ebc5773a0afa6e078b11f4ed7e39a27e0dfc4",
            "sha256:d18a5cc71b84bc4af394a31116bd3932b42241de70c77d2b76d69a314ec8aa12",
        ];

        // Verify layer hash format validation would work
        for hash in &layer_hashes {
            assert!(hash.starts_with("sha256:"));
            assert_eq!(hash.len(), 71); // sha256: + 64 hex characters
        }

        // Ensure request serialization is compatible
        let json = serde_json::to_string(&request).expect("Serialization should succeed");
        assert!(json.contains("specialized-model"));
        assert!(json.contains("domain_expert"));
    }

    #[test]
    fn test_ollama_create_request_status_response_compatibility() {
        let request = OllamaCreateRequest::new(
            "test-model:1.0".to_string(),
            "base-model:latest".to_string(),
            "Test system prompt for status compatibility".to_string(),
        );

        // Test the exact JSON structure from the example
        let example_status_responses = vec![
            r#"{"status": "using existing layer sha256:7f4030143c1c477224c5434f8272c662a8b042079a0a584f0a27a1684fe2e1fa"}"#,
            r#"{"status": "using existing layer sha256:ae370d884f108d16e7cc8fd5259ebc5773a0afa6e078b11f4ed7e39a27e0dfc4"}"#,
            r#"{"status": "creating new layer sha256:941b69ca7dc2a85c053c38d9e8029c9df6224e545060954fa97587f87c044a64"}"#,
            r#"{"status": "writing manifest"}"#,
            r#"{"status": "success"}"#,
        ];

        // Verify each status response can be parsed
        for status_json in &example_status_responses {
            let parsed: serde_json::Value =
                serde_json::from_str(status_json).expect("Status JSON should be valid");
            assert!(parsed.get("status").is_some());
            assert!(parsed["status"].is_string());
        }

        // Verify our request structure is compatible
        let request_json =
            serde_json::to_string(&request).expect("Request serialization should succeed");
        let parsed_request: serde_json::Value =
            serde_json::from_str(&request_json).expect("Request JSON should be valid");

        assert_eq!(parsed_request["model"], "test-model:1.0");
        assert_eq!(parsed_request["name"], "status_test_assistant");
    }

    #[test]
    fn test_ollama_create_request_layer_operation_workflow() {
        // Test a complete workflow simulation
        let requests = vec![
            OllamaCreateRequest::new(
                "assistant-v1".to_string(),
                "llama2:7b".to_string(),
                "You are a helpful assistant for general tasks.".to_string(),
            ),
            OllamaCreateRequest::new(
                "coder-v1".to_string(),
                "codellama:13b".to_string(),
                "You are a specialized coding assistant.".to_string(),
            ),
            OllamaCreateRequest::new(
                "analyst-v1".to_string(),
                "mistral:7b".to_string(),
                "You are a data analysis expert.".to_string(),
            ),
        ];

        for request in &requests {
            // Each request should serialize successfully
            let json = serde_json::to_string(request).expect("Serialization should succeed");

            // Should contain all required fields
            assert!(json.contains("from"));
            assert!(json.contains("system"));
            assert!(json.contains("name"));

            // Should be able to roundtrip
            let deserialized: OllamaCreateRequest =
                serde_json::from_str(&json).expect("Deserialization should succeed");
            assert_eq!(*request, deserialized);
        }

        // Verify different models create different requests
        assert_ne!(requests[0], requests[1]);
        assert_ne!(requests[1], requests[2]);
        assert_ne!(requests[0], requests[2]);
    }
}
