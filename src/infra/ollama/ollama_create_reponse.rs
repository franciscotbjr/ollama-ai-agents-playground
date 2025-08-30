use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct OllamaCreateStatusMessage {
    pub status: String,
}

impl OllamaCreateStatusMessage {
    pub fn new(status: String) -> Self {
        Self { status }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct OllamaCreateResponse {
    pub messages: Vec<OllamaCreateStatusMessage>,
}

impl OllamaCreateResponse {
    pub fn new(messages: Vec<String>) -> Self {
        let status_messages = messages
            .into_iter()
            .map(|msg| OllamaCreateStatusMessage::new(msg))
            .collect();

        Self {
            messages: status_messages,
        }
    }

    pub fn new_with_status_messages(messages: Vec<OllamaCreateStatusMessage>) -> Self {
        Self { messages }
    }

    pub fn from_status_strings(status_strings: Vec<String>) -> Self {
        Self::new(status_strings)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_ollama_create_response_new() {
        let messages = vec![
            "Assistant created successfully".to_string(),
            "Model initialized".to_string(),
        ];
        let response = OllamaCreateResponse::new(messages.clone());

        assert_eq!(response.messages.len(), 2);
        assert_eq!(
            response.messages[0].status,
            "Assistant created successfully"
        );
        assert_eq!(response.messages[1].status, "Model initialized");
    }

    #[test]
    fn test_ollama_create_response_new_empty() {
        let response = OllamaCreateResponse::new(vec![]);

        assert_eq!(response.messages.len(), 0);
        assert!(response.messages.is_empty());
    }

    #[test]
    fn test_ollama_create_response_serialization() {
        let messages = vec!["Success".to_string(), "Ready to use".to_string()];
        let response = OllamaCreateResponse::new(messages);
        let json = serde_json::to_string(&response).expect("Serialization should succeed");

        let expected_json = r#"{"messages":[{"status":"Success"},{"status":"Ready to use"}]}"#;
        assert_eq!(json, expected_json);
    }

    #[test]
    fn test_ollama_create_response_serialization_empty() {
        let response = OllamaCreateResponse::new(vec![]);
        let json = serde_json::to_string(&response).expect("Serialization should succeed");

        let expected_json = r#"{"messages":[]}"#;
        assert_eq!(json, expected_json);
    }

    #[test]
    fn test_ollama_create_response_serialization_single_message() {
        let response = OllamaCreateResponse::new(vec!["Single message".to_string()]);
        let json = serde_json::to_string(&response).expect("Serialization should succeed");

        let expected_json = r#"{"messages":[{"status":"Single message"}]}"#;
        assert_eq!(json, expected_json);
    }

    #[test]
    fn test_ollama_create_response_deserialization() {
        let json = r#"{"messages":[{"status":"Task completed"},{"status":"Model ready"}]}"#;
        let response: OllamaCreateResponse =
            serde_json::from_str(json).expect("Deserialization should succeed");

        assert_eq!(response.messages.len(), 2);
        assert_eq!(response.messages[0].status, "Task completed");
        assert_eq!(response.messages[1].status, "Model ready");
    }

    #[test]
    fn test_ollama_create_response_deserialization_empty() {
        let json = r#"{"messages":[]}"#;
        let response: OllamaCreateResponse =
            serde_json::from_str(json).expect("Deserialization should succeed");

        assert_eq!(response.messages.len(), 0);
        assert!(response.messages.is_empty());
    }

    #[test]
    fn test_ollama_create_response_roundtrip_serialization() {
        let original = OllamaCreateResponse::new(vec![
            "Original message 1".to_string(),
            "Original message 2".to_string(),
            "Original message 3".to_string(),
        ]);
        let json = serde_json::to_string(&original).expect("Serialization should succeed");
        let deserialized: OllamaCreateResponse =
            serde_json::from_str(&json).expect("Deserialization should succeed");

        assert_eq!(original, deserialized);
    }

    #[test]
    fn test_ollama_create_response_with_empty_strings() {
        let messages = vec!["".to_string(), "Valid message".to_string(), "".to_string()];
        let response = OllamaCreateResponse::new(messages.clone());
        let json = serde_json::to_string(&response).expect("Serialization should succeed");
        let deserialized: OllamaCreateResponse =
            serde_json::from_str(&json).expect("Deserialization should succeed");

        assert_eq!(response, deserialized);
        assert_eq!(deserialized.messages.len(), 3);
        assert_eq!(deserialized.messages[0].status, "");
        assert_eq!(deserialized.messages[1].status, "Valid message");
        assert_eq!(deserialized.messages[2].status, "");
    }

    #[test]
    fn test_ollama_create_response_with_unicode_content() {
        let unicode_messages = vec![
            "Hello 世界! 🌍".to_string(),
            "Café naïve résumé".to_string(),
            "测试消息 🤖".to_string(),
        ];
        let response = OllamaCreateResponse::new(unicode_messages.clone());
        let json = serde_json::to_string(&response).expect("Serialization should succeed");
        let deserialized: OllamaCreateResponse =
            serde_json::from_str(&json).expect("Deserialization should succeed");

        assert_eq!(response, deserialized);
        assert_eq!(deserialized.messages[0].status, "Hello 世界! 🌍");
        assert_eq!(deserialized.messages[1].status, "Café naïve résumé");
        assert_eq!(deserialized.messages[2].status, "测试消息 🤖");
    }

    #[test]
    fn test_ollama_create_response_with_special_characters() {
        let special_messages = vec![
            r#"Line 1
Line 2 with "quotes" and 'apostrophes'"#
                .to_string(),
            r#"Tab:	End"#.to_string(),
            r#"Backslash: \ Forward slash: /"#.to_string(),
            r#"JSON special: {"key": "value"}"#.to_string(),
        ];
        let response = OllamaCreateResponse::new(special_messages.clone());
        let json = serde_json::to_string(&response).expect("Serialization should succeed");
        let deserialized: OllamaCreateResponse =
            serde_json::from_str(&json).expect("Deserialization should succeed");

        assert_eq!(response, deserialized);
        for (i, expected_msg) in special_messages.iter().enumerate() {
            assert_eq!(deserialized.messages[i].status, *expected_msg);
        }
    }

    #[test]
    fn test_ollama_create_response_deserialization_invalid_json() {
        let invalid_json = r#"{"messages":"not an array"}"#; // messages should be array
        let result: Result<OllamaCreateResponse, _> = serde_json::from_str(invalid_json);
        assert!(result.is_err());
    }

    #[test]
    fn test_ollama_create_response_deserialization_missing_field() {
        let json_missing_messages = r#"{}"#;
        let result: Result<OllamaCreateResponse, _> = serde_json::from_str(json_missing_messages);
        assert!(result.is_err());
    }

    #[test]
    fn test_ollama_create_response_with_long_messages() {
        let long_message = "This is a very long message that contains many words and should test the serialization and deserialization of longer content. ".repeat(100);
        let response = OllamaCreateResponse::new(vec![
            long_message.clone(),
            "Short message".to_string(),
            long_message.clone(),
        ]);
        let json = serde_json::to_string(&response).expect("Serialization should succeed");
        let deserialized: OllamaCreateResponse =
            serde_json::from_str(&json).expect("Deserialization should succeed");

        assert_eq!(response, deserialized);
        assert_eq!(deserialized.messages[0].status, long_message);
        assert_eq!(deserialized.messages[1].status, "Short message");
        assert_eq!(deserialized.messages[2].status, long_message);
    }

    #[test]
    fn test_ollama_create_response_clone() {
        let original = OllamaCreateResponse::new(vec![
            "Original message 1".to_string(),
            "Original message 2".to_string(),
        ]);
        let cloned = original.clone();

        assert_eq!(original, cloned);
        assert_eq!(original.messages, cloned.messages);

        // Verify they are separate instances
        assert_ne!(
            &original as *const OllamaCreateResponse,
            &cloned as *const OllamaCreateResponse
        );
    }

    #[test]
    fn test_ollama_create_response_partial_eq() {
        let response1 = OllamaCreateResponse::new(vec![
            "Same message 1".to_string(),
            "Same message 2".to_string(),
        ]);
        let response2 = OllamaCreateResponse::new(vec![
            "Same message 1".to_string(),
            "Same message 2".to_string(),
        ]);
        let response3 = OllamaCreateResponse::new(vec!["Different message".to_string()]);
        let response4 = OllamaCreateResponse::new(vec![
            "Same message 1".to_string(),
            "Different message 2".to_string(),
        ]);
        let response5 = OllamaCreateResponse::new(vec![]); // Empty

        assert_eq!(response1, response2);
        assert_ne!(response1, response3);
        assert_ne!(response1, response4);
        assert_ne!(response1, response5);
    }

    #[test]
    fn test_ollama_create_response_debug_format() {
        let response = OllamaCreateResponse::new(vec![
            "Debug message 1".to_string(),
            "Debug message 2".to_string(),
        ]);
        let debug_string = format!("{:?}", response);

        assert!(debug_string.contains("OllamaCreateResponse"));
        assert!(debug_string.contains("Debug message 1"));
        assert!(debug_string.contains("Debug message 2"));
    }

    #[test]
    fn test_ollama_create_response_deserialization_extra_fields() {
        let json_extra_fields = r#"{"messages":[{"status":"Test message"}],"extra":"ignored"}"#;
        let result: Result<OllamaCreateResponse, _> = serde_json::from_str(json_extra_fields);
        assert!(result.is_ok());

        let response = result.unwrap();
        assert_eq!(response.messages.len(), 1);
        assert_eq!(response.messages[0].status, "Test message");
    }

    #[test]
    fn test_ollama_create_response_with_large_array() {
        let messages: Vec<String> = (0..1000).map(|i| format!("Message number {}", i)).collect();
        let response = OllamaCreateResponse::new(messages.clone());
        let json = serde_json::to_string(&response).expect("Serialization should succeed");
        let deserialized: OllamaCreateResponse =
            serde_json::from_str(&json).expect("Deserialization should succeed");

        assert_eq!(response, deserialized);
        assert_eq!(deserialized.messages.len(), 1000);
        assert_eq!(deserialized.messages[0].status, "Message number 0");
        assert_eq!(deserialized.messages[999].status, "Message number 999");
    }

    #[test]
    fn test_ollama_create_response_consistency() {
        let messages = vec![
            "Consistent message 1".to_string(),
            "Consistent message 2".to_string(),
        ];

        let response = OllamaCreateResponse::new(messages.clone());

        assert_eq!(response.messages.len(), messages.len());
        for (i, expected_msg) in messages.iter().enumerate() {
            assert_eq!(response.messages[i].status, *expected_msg);
        }
    }

    // Concrete test cases based on the real Ollama create response example
    #[test]
    fn test_ollama_create_response_with_layer_statuses() {
        let layer_statuses = vec![
            "using existing layer sha256:7f4030143c1c477224c5434f8272c662a8b042079a0a584f0a27a1684fe2e1fa".to_string(),
            "using existing layer sha256:ae370d884f108d16e7cc8fd5259ebc5773a0afa6e078b11f4ed7e39a27e0dfc4".to_string(),
            "creating new layer sha256:941b69ca7dc2a85c053c38d9e8029c9df6224e545060954fa97587f87c044a64".to_string(),
            "writing manifest".to_string(),
            "success".to_string()
        ];

        let response = OllamaCreateResponse::new(layer_statuses.clone());

        assert_eq!(response.messages.len(), 5);
        assert_eq!(
            response.messages[0].status,
            "using existing layer sha256:7f4030143c1c477224c5434f8272c662a8b042079a0a584f0a27a1684fe2e1fa"
        );
        assert_eq!(
            response.messages[1].status,
            "using existing layer sha256:ae370d884f108d16e7cc8fd5259ebc5773a0afa6e078b11f4ed7e39a27e0dfc4"
        );
        assert_eq!(
            response.messages[2].status,
            "creating new layer sha256:941b69ca7dc2a85c053c38d9e8029c9df6224e545060954fa97587f87c044a64"
        );
        assert_eq!(response.messages[3].status, "writing manifest");
        assert_eq!(response.messages[4].status, "success");

        // Test serialization
        let json = serde_json::to_string(&response).expect("Serialization should succeed");
        assert!(json.contains("\"status\":\"using existing layer"));
        assert!(json.contains("\"status\":\"creating new layer"));
        assert!(json.contains("\"status\":\"writing manifest\""));
        assert!(json.contains("\"status\":\"success\""));
    }

    #[test]
    fn test_ollama_create_response_exact_json_format() {
        // Test the exact JSON structure from the example
        let json_input = r#"{"messages":[
            {"status": "using existing layer sha256:7f4030143c1c477224c5434f8272c662a8b042079a0a584f0a27a1684fe2e1fa"},
            {"status": "using existing layer sha256:ae370d884f108d16e7cc8fd5259ebc5773a0afa6e078b11f4ed7e39a27e0dfc4"},
            {"status": "creating new layer sha256:941b69ca7dc2a85c053c38d9e8029c9df6224e545060954fa97587f87c044a64"},
            {"status": "writing manifest"},
            {"status": "success"}
        ]}"#;

        let response: OllamaCreateResponse =
            serde_json::from_str(json_input).expect("Deserialization should succeed");

        assert_eq!(response.messages.len(), 5);
        assert!(response.messages[0].status.contains("sha256:7f4030143c1c"));
        assert!(response.messages[1].status.contains("sha256:ae370d884f10"));
        assert!(response.messages[2].status.contains("creating new layer"));
        assert_eq!(response.messages[3].status, "writing manifest");
        assert_eq!(response.messages[4].status, "success");

        // Test round-trip serialization
        let json_output = serde_json::to_string(&response).expect("Serialization should succeed");
        let roundtrip_response: OllamaCreateResponse =
            serde_json::from_str(&json_output).expect("Round-trip deserialization should succeed");

        assert_eq!(response, roundtrip_response);
    }

    #[test]
    fn test_ollama_create_response_status_message_struct() {
        // Test the OllamaCreateStatusMessage struct directly
        let status_msg = OllamaCreateStatusMessage::new("test status".to_string());
        assert_eq!(status_msg.status, "test status");

        // Test serialization of individual status message
        let json = serde_json::to_string(&status_msg).expect("Serialization should succeed");
        assert_eq!(json, r#"{"status":"test status"}"#);

        // Test deserialization of individual status message
        let deserialized: OllamaCreateStatusMessage =
            serde_json::from_str(&json).expect("Deserialization should succeed");
        assert_eq!(deserialized, status_msg);
    }

    #[test]
    fn test_ollama_create_response_new_with_status_messages() {
        let status_messages = vec![
            OllamaCreateStatusMessage::new("first status".to_string()),
            OllamaCreateStatusMessage::new("second status".to_string()),
            OllamaCreateStatusMessage::new("third status".to_string()),
        ];

        let response = OllamaCreateResponse::new_with_status_messages(status_messages.clone());

        assert_eq!(response.messages.len(), 3);
        assert_eq!(response.messages, status_messages);
    }

    #[test]
    fn test_ollama_create_response_from_status_strings() {
        let status_strings = vec![
            "layer operation 1".to_string(),
            "layer operation 2".to_string(),
            "manifest written".to_string(),
        ];

        let response = OllamaCreateResponse::from_status_strings(status_strings.clone());

        assert_eq!(response.messages.len(), 3);
        assert_eq!(response.messages[0].status, "layer operation 1");
        assert_eq!(response.messages[1].status, "layer operation 2");
        assert_eq!(response.messages[2].status, "manifest written");
    }
}
