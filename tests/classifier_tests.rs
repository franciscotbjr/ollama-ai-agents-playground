use ollama_ai_agents_playground::agents::{
    Intent,
    classifier::{ClassificationResult, IntentParam, Params, map_ollama_to_classification},
};
use ollama_oxide::ResponseMessage;

fn make_response_message(content: &str) -> ResponseMessage {
    serde_json::from_str(&format!(
        r#"{{"role": "assistant", "content": "{}"}}"#,
        content.replace('"', r#"\""#).replace('\n', r#"\n"#)
    ))
    .unwrap()
}

// --- Pipeline: ResponseMessage → map → ClassificationResult ---

#[test]
fn test_pipeline_send_email_intent() {
    let content = r#"```json
{
  "intent": "send_email",
  "params": {
    "recipient": "eva@company.com",
    "message": "Não vou poder comparecer à reunião"
  }
}
```"#;
    let response = make_response_message(content);
    let result = map_ollama_to_classification(&response).unwrap();

    assert_eq!(result.intent, Intent::SendEmail);
    assert_eq!(result.params.recipient(), Some("eva@company.com"));
    assert!(result.params.message().unwrap().contains("reunião"));
}

#[test]
fn test_pipeline_schedule_meeting_intent() {
    let content = r#"```json
{
  "intent": "schedule_meeting",
  "params": {
    "recipient": "alice@team.com",
    "message": "Sprint review às 14h"
  }
}
```"#;
    let response = make_response_message(content);
    let result = map_ollama_to_classification(&response).unwrap();

    assert_eq!(result.intent, Intent::ScheduleMeeting);
    assert_eq!(result.params.recipient(), Some("alice@team.com"));
    assert!(result.params.message().unwrap().contains("Sprint review"));
}

#[test]
fn test_pipeline_no_action_intent() {
    let content = r#"```json
{
  "intent": "no_action",
  "params": {
    "recipient": null,
    "message": null
  }
}
```"#;
    let response = make_response_message(content);
    let result = map_ollama_to_classification(&response).unwrap();

    assert_eq!(result.intent, Intent::NoAction);
    assert_eq!(result.params.recipient(), None);
    assert_eq!(result.params.message(), None);
}

#[test]
fn test_pipeline_plain_json_no_markdown() {
    let content = r#"{
  "intent": "send_email",
  "params": {
    "recipient": "plain@example.com",
    "message": "Texto sem markdown"
  }
}"#;
    let response = make_response_message(content);
    let result = map_ollama_to_classification(&response).unwrap();

    assert_eq!(result.intent, Intent::SendEmail);
    assert_eq!(result.params.recipient(), Some("plain@example.com"));
}

#[test]
fn test_pipeline_returns_error_on_invalid_content() {
    let content = "Não sei o que fazer com isso.";
    let response = make_response_message(content);
    let result = map_ollama_to_classification(&response);

    assert!(result.is_err());
}

// --- ClassificationResult: serialization roundtrip ---

#[test]
fn test_classification_result_roundtrip() {
    let params = Params::with_values("a@b.com".to_string(), "msg".to_string());
    let original = ClassificationResult::new(Intent::SendEmail, params);

    let json = original.to_json_string().unwrap();
    let restored = ClassificationResult::from_json_str(&json).unwrap();

    assert_eq!(restored.intent, original.intent);
    assert_eq!(restored.params.recipient(), original.params.recipient());
    assert_eq!(restored.params.message(), original.params.message());
}

#[test]
fn test_classification_result_all_intents_roundtrip() {
    for intent in [Intent::SendEmail, Intent::ScheduleMeeting, Intent::NoAction] {
        let params = Params::new(None, None);
        let original = ClassificationResult::new(intent.clone(), params);

        let json = original.to_json_string().unwrap();
        let restored = ClassificationResult::from_json_str(&json).unwrap();

        assert_eq!(restored.intent, original.intent);
    }
}

// --- IntentParam: accessor integration ---

#[test]
fn test_intent_param_carries_input_and_assistant() {
    let input = "Envie um email para João dizendo que a reunião foi cancelada".to_string();
    let assistant = "assistant-tereza".to_string();
    let param = IntentParam::new(input.clone(), assistant.clone());

    assert_eq!(param.input(), input.as_str());
    assert_eq!(param.assistant(), assistant.as_str());
}

#[test]
fn test_intent_param_with_unicode() {
    let input = "Agende uma reunião com María às 10h de terça-feira".to_string();
    let param = IntentParam::new(input.clone(), "assistente".to_string());

    assert_eq!(param.input(), input.as_str());
    assert!(param.input().contains("María"));
    assert!(param.input().contains("terça-feira"));
}

// --- Params: integration with ClassificationResult ---

#[test]
fn test_params_integrated_with_result() {
    let params = Params::with_values(
        "recipient@domain.com".to_string(),
        "Confirmo presença na reunião".to_string(),
    );
    let result = ClassificationResult::new(Intent::SendEmail, params);

    assert_eq!(result.params.recipient(), Some("recipient@domain.com"));
    assert!(result.params.message().unwrap().contains("reunião"));

    let json = result.to_json_string().unwrap();
    assert!(json.contains("send_email"));
    assert!(json.contains("recipient@domain.com"));
}

#[test]
fn test_params_null_values_in_result() {
    let params = Params::new(None, None);
    let result = ClassificationResult::new(Intent::NoAction, params);

    assert_eq!(result.params.recipient(), None);
    assert_eq!(result.params.message(), None);

    let json = result.to_json_string().unwrap();
    assert!(json.contains("no_action"));
}

// --- Complete end-to-end scenario ---

#[test]
fn test_full_scenario_email_to_eva() {
    // Simulates the real use case from main.rs:
    // "Envie um e-mail para Eva informando que não vou poder comparecer..."
    let ollama_mock = r#"```json
{
  "intent": "send_email",
  "params": {
    "recipient": "Eva",
    "message": "Não vou poder comparecer à reunião e peço desculpas por avisar tão em cima da hora"
  }
}
```"#;

    let response = make_response_message(ollama_mock);
    let result = map_ollama_to_classification(&response).unwrap();

    assert_eq!(result.intent, Intent::SendEmail);
    assert_eq!(result.params.recipient(), Some("Eva"));
    let message = result.params.message().unwrap();
    assert!(message.contains("reunião"));
    assert!(message.contains("desculpas"));
}
