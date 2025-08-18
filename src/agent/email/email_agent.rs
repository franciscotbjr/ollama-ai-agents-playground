use crate::agent::{Agent, AgentError, ClassificationResult};

pub struct EmailAgent {}

impl EmailAgent {
    pub fn new() -> Self {
        Self {}
    }
}

impl Agent for EmailAgent {
    fn process(
        &self,
        _input: &str,
    ) -> impl std::future::Future<Output = Result<ClassificationResult, AgentError>> + Send {
        async move {
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
}
