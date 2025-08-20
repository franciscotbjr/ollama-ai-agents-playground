use crate::agent::{
    Agent, AgentError, contact::contact_result::ContactResult,
};

pub struct ContactAgent {}

impl ContactAgent {
    pub fn new() -> Self {
        Self {}
    }
}

impl Agent<ContactResult> for ContactAgent {
    fn process(
        &self,
        _input: &str,
    ) -> impl std::future::Future<Output = Result<ContactResult, AgentError>> + Send {
        async move {
            // TODO: Implement contact data discovery logic

            Err(AgentError::ProcessingError(
                "Contact finding not implemented yet".to_string(),
            ))
        }
    }
}
