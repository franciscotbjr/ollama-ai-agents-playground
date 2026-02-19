use ollama_ai_agents_playground::agents::assistant::{
    CheckParam, CheckResult, CreateParam, CreateResult, build_assistant_name,
};

// --- CheckParam ---

#[test]
fn test_check_param_stores_and_exposes_name() {
    let param = CheckParam::new("assistant-tereza".to_string());
    assert_eq!(param.name(), "assistant-tereza");
}

#[test]
fn test_check_param_with_empty_name() {
    let param = CheckParam::new(String::new());
    assert_eq!(param.name(), "");
}

#[test]
fn test_check_param_with_unicode_name() {
    let name = "assistant-助理-Tereza".to_string();
    let param = CheckParam::new(name.clone());
    assert_eq!(param.name(), name.as_str());
}

// --- CheckResult ---

#[test]
fn test_check_result_exists_true() {
    let result = CheckResult::new(true);
    assert!(result.exists);
}

#[test]
fn test_check_result_exists_false() {
    let result = CheckResult::new(false);
    assert!(!result.exists);
}

#[test]
fn test_check_result_serialization_roundtrip() {
    let original = CheckResult::new(true);

    let json = serde_json::to_string(&original).unwrap();
    let restored: CheckResult = serde_json::from_str(&json).unwrap();

    assert_eq!(restored.exists, original.exists);
}

#[test]
fn test_check_result_default_exists_is_false() {
    let json = r#"{}"#;
    let result: CheckResult = serde_json::from_str(json).unwrap();
    assert!(!result.exists);
}

// --- CreateParam ---

#[test]
fn test_create_param_stores_and_exposes_fields() {
    let param = CreateParam::new("Ana".to_string(), "Tereza".to_string());
    assert_eq!(param.assistant_to(), "Ana");
    assert_eq!(param.name(), "Tereza");
}

#[test]
fn test_create_param_with_empty_fields() {
    let param = CreateParam::new(String::new(), String::new());
    assert_eq!(param.assistant_to(), "");
    assert_eq!(param.name(), "");
}

#[test]
fn test_create_param_with_unicode() {
    let param = CreateParam::new("María García".to_string(), "Tereza-助理".to_string());
    assert_eq!(param.assistant_to(), "María García");
    assert_eq!(param.name(), "Tereza-助理");
}

// --- CreateResult ---

#[test]
fn test_create_result_success_true() {
    let result = CreateResult::new(true);
    assert!(result.success);
}

#[test]
fn test_create_result_success_false() {
    let result = CreateResult::new(false);
    assert!(!result.success);
}

#[test]
fn test_create_result_serialization_roundtrip() {
    let original = CreateResult::new(true);

    let json = serde_json::to_string(&original).unwrap();
    let restored: CreateResult = serde_json::from_str(&json).unwrap();

    assert_eq!(restored.success, original.success);
}

#[test]
fn test_create_result_default_success_is_false() {
    let json = r#"{}"#;
    let result: CreateResult = serde_json::from_str(json).unwrap();
    assert!(!result.success);
}

// --- build_assistant_name (requires config.toml) ---

#[test]
fn test_build_assistant_name_appends_to_root_prefix() {
    // From config.toml: assistant.root.name = "assistant"
    // Expected: "assistant-{name}"
    let name = build_assistant_name("tereza");
    assert!(name.contains("tereza"), "Name should contain the input: {name}");
    assert!(name.contains('-'), "Name should have a separator: {name}");
}

#[test]
fn test_build_assistant_name_different_inputs_produce_different_names() {
    let name_a = build_assistant_name("tereza");
    let name_b = build_assistant_name("sofia");

    assert_ne!(name_a, name_b);
    assert!(name_a.contains("tereza"));
    assert!(name_b.contains("sofia"));
}

// --- Cross-module: CheckParam → CheckResult integration ---

#[test]
fn test_check_param_and_result_integration() {
    let param = CheckParam::new(build_assistant_name("tereza"));
    let result = CheckResult::new(true);

    assert!(!param.name().is_empty());
    assert!(result.exists);
    assert!(param.name().contains("tereza"));
}

// --- Cross-module: CreateParam → CreateResult integration ---

#[test]
fn test_create_param_and_result_integration() {
    let param = CreateParam::new("Ana".to_string(), "Tereza".to_string());
    let result = CreateResult::new(true);

    assert_eq!(param.assistant_to(), "Ana");
    assert_eq!(param.name(), "Tereza");
    assert!(result.success);
}
