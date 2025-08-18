use crate::{
    agent::{
        Agent, AgentError, ClassificationResult,
        classifier::{ClassifierPrompt, ToClassificationResult},
    },
    infra::ollama::{OllamaClient, OllamaResponse},
};

pub struct ClassifierAgent {}

impl Agent for ClassifierAgent {
    fn process(
        &self,
        input: &str,
    ) -> impl std::future::Future<Output = Result<ClassificationResult, AgentError>> + Send {
        async move {
            // Build classification prompt
            let prompt = build_prompt(input);

            // Send to Ollama API
            let ollama_response: OllamaResponse =
                OllamaClient::send_message(&prompt.as_str()).await;

            // Parse JSON response and convert to ClassificationResult
            match ollama_response.message.to_classification_result() {
                Ok(classification_result) => Ok(classification_result),
                Err(mapper_error) => Err(AgentError::ParseError(format!(
                    "Failed to parse Ollama response: {}",
                    mapper_error
                ))),
            }
        }
    }
}

fn build_prompt(input: &str) -> String {
    ClassifierPrompt::builder()
        .add_instruction(CLASSIFY_INTENT_TO_JSON)
        .add_instruction(SPACE)
        .add_instruction(EXAMPLE_1)
        .add_instruction(SPACE)
        .add_instruction(EXAMPLE_2)
        .add_instruction(SPACE)
        .add_instruction(TASK)
        .add_instruction(SPACE)
        .add_instruction(INPUT.replace("{}", input).as_str())
        .add_instruction(SPACE)
        .add_instruction(OUTPUT)
        .build()
        .content()
        .to_string()
}

const SPACE: &str = "        ";
const CLASSIFY_INTENT_TO_JSON: &str = "Classify intent and extract parameters (JSON format):";
const EXAMPLE_1: &str = "Example 1:        Input: \"Send an email to Carlos about the delay\"        Output: {\"intent\":\"send_email\", \"params\":{\"recipient\":\"Carlos\",\"message\":\"About the delay\"}}";
const EXAMPLE_2: &str = "Example 2:        Input: \"Send message to Sofia: I'll arrive in 10 min\"        Output: {\"intent\":\"send_message\", \"params\":{\"recipient\":\"Sofia\",\"message\":\"I'll arrive in 10 min\"}}";
const TASK: &str = "Task: Return JSON with: action (send_email, schedule_meeting, no_action)";
const INPUT: &str = "Input: \"{}\"";
const OUTPUT: &str = "Output: ";
