use crate::{agent::{classifier::{ClassifierPrompt, ToClassificationResult}, Agent, AgentError, ClassificationResult}, infra::ollama::{OllamaClient, OllamaResponse}};

pub struct ClassifierAgent {}

impl Agent for ClassifierAgent {
    async fn process(&self, input: &str) -> Result<ClassificationResult, AgentError> {
        // TODO: Implement classification logic with Ollama
        // - Build classification prompt
        let prompt = build_prompt(input);
        // - Send to Ollama API
        // - Parse JSON response
        let ollamaResponse: OllamaResponse = OllamaClient::send_message(&prompt.as_str()).await;
        // - Return serialized ClassificationResult
        ollamaResponse.message.to_classification_result()
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
            .content().to_string()
}


const SPACE: &str = "        ";
const CLASSIFY_INTENT_TO_JSON : &str = "Classify intent and extract parameters (JSON format):";
const EXAMPLE_1 : &str = "Example 1:        Input: \"Send an email to Carlos about the delay\"        Output: {\"intent\":\"send_email\", \"params\":{\"recipient\":\"Carlos\",\"message\":\"About the delay\"}}";
const EXAMPLE_2 : &str = "Example 2:        Input: \"Send message to Sofia: I'll arrive in 10 min\"        Output: {\"intent\":\"send_message\", \"params\":{\"recipient\":\"Sofia\",\"message\":\"I'll arrive in 10 min\"}}";
const TASK : &str = "Task: Return JSON with: action (send_email, schedule_meeting, no_action)";
const INPUT : &str = "Input: \"{}\"";
const OUTPUT : &str = "Output: ";