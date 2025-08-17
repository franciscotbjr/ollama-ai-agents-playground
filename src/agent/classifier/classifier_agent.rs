use crate::agent::{Agent, AgentError};

pub struct ClassifierAgent {}

impl Agent for ClassifierAgent {
    async fn process(&self, input: &str) -> Result<String, AgentError> {
        // TODO: Implement classification logic with Ollama
        // - Build classification prompt
        // - Send to Ollama API
        // - Parse JSON response
        // - Return serialized ClassificationResult

        Err(AgentError::ProcessingError(
            "Not implemented yet".to_string(),
        ))
    }
}
