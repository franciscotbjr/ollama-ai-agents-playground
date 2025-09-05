use crate::{
    agent::{
        Agent, AgentError, ClassificationResult, agent::AgentParam, agent_prompt::AgentPrompt,
        classifier::ToClassificationResult,
    },
    infra::ollama::OllamaClient,
};

pub struct IntentClassifierAgent {}

impl IntentClassifierAgent {
    pub fn new() -> Self {
        Self {}
    }
}

pub struct IntentParam {
    input: String,
    assistant: String,
}

impl IntentParam {
    pub fn new(input: String, assistant: String) -> Self {
        Self { input, assistant }
    }
}

impl AgentParam for IntentParam {}

impl Agent<IntentParam, ClassificationResult> for IntentClassifierAgent {
    fn process(
        &self,
        input: IntentParam,
    ) -> impl std::future::Future<Output = Result<ClassificationResult, AgentError>> + Send {
        async move {
            // Build classification prompt
            let prompt = build_prompt(&input.input);

            // Send to Ollama API
            let result = OllamaClient::new()
                .send_message(&prompt.as_str(), &input.assistant)
                .await;

            match result {
                Ok(ollama_response) => {
                    // Parse JSON response and convert to ClassificationResult
                    match ollama_response.message.to_classification_result() {
                        Ok(classification_result) => Ok(classification_result),
                        Err(mapper_error) => Err(AgentError::ParseError(format!(
                            "Classification failed: {}",
                            mapper_error
                        ))),
                    }
                }
                Err(e) => Err(AgentError::ParseError(format!(
                    "Classification failed: {}",
                    e
                ))),
            }
        }
    }
}

fn build_prompt(input: &str) -> String {
    AgentPrompt::builder()
        .add_instruction(CLASSIFY_INTENT_TO_JSON)
        .add_instruction(SPACE)
        .add_instruction(OUTPUT_FORMART)
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
const OUTPUT_FORMART: &str =
    "Output-Format: {\"intent\":\"\",\"params\":{\"recipient\":\"\",\"message\":\"\"}}";
const EXAMPLE_1: &str = "Example 1:        Input: \"Send an email to Carlos about the delay\"        Output: {\"intent\":\"send_email\", \"params\":{\"recipient\":\"Carlos\",\"message\":\"About the delay\"}}";
const EXAMPLE_2: &str = "Example 2:        Input: \"Send message to Sofia: I'll arrive in 10 min\"        Output: {\"intent\":\"send_message\", \"params\":{\"recipient\":\"Sofia\",\"message\":\"I'll arrive in 10 min\"}}";
const TASK: &str = "Task: Return JSON with: action (send_email, schedule_meeting, no_action)";
const INPUT: &str = "Input: \"{}\"";
const OUTPUT: &str = "Output: ";
