use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct OllamaCheckResult {
    pub exists: bool,
}

impl OllamaCheckResult {
    pub fn new(exists: bool) -> Self {
        Self { exists }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_ollama_check_result_new_true() {
        let result = OllamaCheckResult::new(true);
        assert_eq!(result.exists, true);
    }

    #[test]
    fn test_ollama_check_result_new_false() {
        let result = OllamaCheckResult::new(false);
        assert_eq!(result.exists, false);
    }

    #[test]
    fn test_ollama_check_result_serialization() {
        let result = OllamaCheckResult::new(true);
        let serialized = serde_json::to_string(&result).expect("Serialization should succeed");

        let expected = r#"{"exists":true}"#;
        assert_eq!(serialized, expected);
    }

    #[test]
    fn test_ollama_check_result_serialization_false() {
        let result = OllamaCheckResult::new(false);
        let serialized = serde_json::to_string(&result).expect("Serialization should succeed");

        let expected = r#"{"exists":false}"#;
        assert_eq!(serialized, expected);
    }

    #[test]
    fn test_ollama_check_result_deserialization_true() {
        let json = r#"{"exists":true}"#;
        let result: OllamaCheckResult =
            serde_json::from_str(json).expect("Deserialization should succeed");

        assert_eq!(result.exists, true);
    }

    #[test]
    fn test_ollama_check_result_deserialization_false() {
        let json = r#"{"exists":false}"#;
        let result: OllamaCheckResult =
            serde_json::from_str(json).expect("Deserialization should succeed");

        assert_eq!(result.exists, false);
    }

    #[test]
    fn test_ollama_check_result_roundtrip_serialization() {
        let original = OllamaCheckResult::new(true);
        let serialized = serde_json::to_string(&original).expect("Serialization should succeed");
        let deserialized: OllamaCheckResult =
            serde_json::from_str(&serialized).expect("Deserialization should succeed");

        assert_eq!(original, deserialized);
    }

    #[test]
    fn test_ollama_check_result_clone() {
        let original = OllamaCheckResult::new(true);
        let cloned = original.clone();

        assert_eq!(original, cloned);
        assert_eq!(cloned.exists, true);
    }

    #[test]
    fn test_ollama_check_result_partial_eq() {
        let result1 = OllamaCheckResult::new(true);
        let result2 = OllamaCheckResult::new(true);
        let result3 = OllamaCheckResult::new(false);

        assert_eq!(result1, result2);
        assert_ne!(result1, result3);
        assert_ne!(result2, result3);
    }

    #[test]
    fn test_ollama_check_result_debug_format() {
        let result = OllamaCheckResult::new(false);
        let debug_string = format!("{:?}", result);

        assert!(debug_string.contains("OllamaCheckResult"));
        assert!(debug_string.contains("exists: false"));
    }
}
