use crate::agent::{Agent, AgentError, ClassificationResult};

pub struct EmailAgent {}

impl EmailAgent {
    pub fn new() -> Self {
        Self {}
    }
}

impl Agent for EmailAgent {
    async fn process(&self, input: &str) -> Result<ClassificationResult, AgentError> {
        // TODO: Implement email sending logic
        // - Parse input parameters (recipient, message)
        // - Validate email address
        // - Send email using email_sender
        // - Return sending confirmation

        Err(AgentError::ProcessingError(
            "Email sending not implemented yet".to_string(),
        ))
    }
}
