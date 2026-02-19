use ollama_oxide::{ChatMessage, ResponseMessage};

use crate::{
    agents::{
        Agent, AgentError, ClassificationResult,
        agent_prompt::AgentPrompt,
        assistant::build_assistant_name,
        classifier::FromMarkdownJson,
    },
    infra::assistant_ollama_client::AssistantOllamaClient,
};

use super::intent_param::IntentParam;

#[derive(Debug, Default)]
pub struct IntentClassifierAgent {}

impl IntentClassifierAgent {
    pub fn new() -> Self {
        Self {}
    }
}

impl Agent<IntentParam, ClassificationResult> for IntentClassifierAgent {
    fn process(
        &self,
        input: IntentParam,
    ) -> impl std::future::Future<Output = Result<ClassificationResult, AgentError>> + Send {
        async move {
            // Build classification prompt
            let systen_prompt = build_system_prompt();

            let user_prompt = build_user_prompt(input.input());

            // Send to Ollama API
            let result = AssistantOllamaClient::new()
                .send_classifier_message(
                    vec![
                        ChatMessage::system(format!(r#"{}"#, systen_prompt.replace('"', "\\\""))),
                        ChatMessage::user(format!(r#"{}"#, user_prompt.replace('"', "\\\""))),
                    ],
                    build_assistant_name(input.assistant()).as_str(),
                )
                .await;

            match result {
                Ok(ollama_response) => {
                    // Parse JSON response and convert to ClassificationResult
                    match ResponseMessage::from_markdown_text(
                        &ollama_response.message.unwrap().content().unwrap(),
                    )
                    {
                        Ok(classification_result) => Ok(classification_result),
                        Err(mapper_error) => Err(AgentError::ParseError(format!(
                            "Classification failed: {}",
                            mapper_error
                        ))),
                    }
                }
                Err(e) => Err(AgentError::ParseError(format!(
                    "Classification failed: {e}"
                ))),
            }
        }
    }
}

fn build_user_prompt(input: &str) -> String {
    AgentPrompt::builder()
        .add_instruction(INPUT.replace("{}", input).as_str())
        .add_instruction(SPACE)
        .add_instruction(OUTPUT)
        .build()
        .content()
        .to_string()
}

fn build_system_prompt() -> String {
    AgentPrompt::builder()
        .add_instruction(YOUT_ARE)
        .add_instruction(SPACE)
        .add_instruction(OUTPUT_FORMART)
        .add_instruction(SPACE)
        .add_instruction(EXAMPLE_1)
        .add_instruction(SPACE)
        .add_instruction(EXAMPLE_2)
        .add_instruction(SPACE)
        .add_instruction(TASK)
        .build()
        .content()
        .to_string()
}

const SPACE: &str = "        ";
const YOUT_ARE: &str = "You are a helpful assistant that classifies user intent. Your response MUST be a JSON object. You will never use markdown notation in outputs. Here are some examples:";
const OUTPUT_FORMART: &str =
    "Output-Format: {\"intent\":\"\",\"params\":{\"recipient\":\"\",\"message\":\"\"}}";
const EXAMPLE_1: &str = "Example 1:        Input: \"Send an email to Carlos about the delay\"        Output: {\"intent\":\"send_email\", \"params\":{\"recipient\":\"Carlos\",\"message\":\"About the delay\"}}";
const EXAMPLE_2: &str = "Example 2:        Input: \"Send message to Sofia: I'll arrive in 10 min\"        Output: {\"intent\":\"send_message\", \"params\":{\"recipient\":\"Sofia\",\"message\":\"I'll arrive in 10 min\"}}";
const TASK: &str = "Task: Return JSON with: action (send_email, schedule_meeting, no_action)";
const INPUT: &str = "Input: \"{}\"";
const OUTPUT: &str = "Output: ";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_intent_classifier_agent_new() {
        let agent = IntentClassifierAgent::new();
        assert!(format!("{:?}", agent).contains("IntentClassifierAgent"));
    }

    #[test]
    fn test_intent_classifier_agent_default() {
        let agent = IntentClassifierAgent::default();
        assert!(format!("{:?}", agent).contains("IntentClassifierAgent"));
    }

    #[test]
    fn test_build_user_prompt() {
        let input = "Send an email to Eva";
        let result = build_user_prompt(input);

        assert!(result.contains("Send an email to Eva"));
        assert!(result.contains("Input:"));
        assert!(result.contains("Output:"));
    }

    #[test]
    fn test_build_user_prompt_with_special_characters() {
        let input = "Send email with \"quotes\" and 'apostrophes'";
        let result = build_user_prompt(input);

        assert!(result.contains("quotes"));
        assert!(result.contains("apostrophes"));
    }

    #[test]
    fn test_build_user_prompt_with_unicode() {
        let input = "Envie um email para João sobre café";
        let result = build_user_prompt(input);

        assert!(result.contains("João"));
        assert!(result.contains("café"));
    }

    #[test]
    fn test_build_system_prompt() {
        let result = build_system_prompt();

        assert!(result.contains("You are a helpful assistant"));
        assert!(result.contains("JSON object"));
        assert!(result.contains("Example 1"));
        assert!(result.contains("Example 2"));
        assert!(result.contains("send_email"));
        assert!(result.contains("schedule_meeting"));
        assert!(result.contains("no_action"));
    }

    #[test]
    fn test_build_system_prompt_contains_format() {
        let result = build_system_prompt();

        assert!(result.contains("Output-Format"));
        assert!(result.contains("intent"));
        assert!(result.contains("params"));
        assert!(result.contains("recipient"));
        assert!(result.contains("message"));
    }

    #[test]
    fn test_build_system_prompt_contains_examples() {
        let result = build_system_prompt();

        assert!(result.contains("Carlos"));
        assert!(result.contains("Sofia"));
        assert!(result.contains("delay"));
        assert!(result.contains("arrive in 10 min"));
    }

    #[test]
    fn test_constants_values() {
        assert_eq!(SPACE, "        ");
        assert!(YOUT_ARE.contains("helpful assistant"));
        assert!(OUTPUT_FORMART.contains("intent"));
        assert!(OUTPUT_FORMART.contains("params"));
        assert!(EXAMPLE_1.contains("Carlos"));
        assert!(EXAMPLE_2.contains("Sofia"));
        assert!(TASK.contains("send_email"));
        assert!(TASK.contains("schedule_meeting"));
        assert!(TASK.contains("no_action"));
        assert_eq!(INPUT, "Input: \"{}\"");
        assert_eq!(OUTPUT, "Output: ");
    }

    #[test]
    fn test_build_user_prompt_formatting() {
        let input = "Test message";
        let result = build_user_prompt(input);

        assert!(result.contains("\"Test message\""));

        let lines: Vec<&str> = result.lines().collect();
        assert!(lines.len() > 0);

        assert!(result.contains("Output: "));
    }

    #[test]
    fn test_build_system_prompt_no_markdown() {
        let result = build_system_prompt();

        assert!(!result.contains("```"));
        assert!(!result.contains("**"));
        assert!(!result.contains("##"));
        assert!(!result.contains("###"));

        assert!(result.contains("never use markdown"));
    }

    #[test]
    fn test_input_string_replacement() {
        let test_input = "This is a test input";
        let formatted = INPUT.replace("{}", test_input);
        assert_eq!(formatted, "Input: \"This is a test input\"");
    }

    #[test]
    fn test_input_string_replacement_with_special_chars() {
        let test_input = "Input with \"quotes\" and {braces}";
        let formatted = INPUT.replace("{}", test_input);
        assert_eq!(formatted, "Input: \"Input with \"quotes\" and {braces}\"");
    }

    #[test]
    fn test_prompt_builder_integration() {
        let input = "Send email to test@example.com";
        let user_prompt = build_user_prompt(input);
        let system_prompt = build_system_prompt();

        assert!(!user_prompt.is_empty());
        assert!(!system_prompt.is_empty());

        assert!(system_prompt.contains("assistant"));
        assert!(system_prompt.contains("JSON"));

        assert!(user_prompt.contains("test@example.com"));
    }
}
