use crate::agents::{Agent, AgentError, agent::AgentParam, contact::contact_result::ContactResult};

pub struct ContactAgent {}

impl ContactAgent {
    pub fn new() -> Self {
        Self {}
    }
}
pub struct ContactParam {
    input: String,
}

impl ContactParam {
    pub fn new(input: &String) -> Self {
        Self {
            input: input.clone(),
        }
    }
}

impl AgentParam for ContactParam {}

impl Agent<ContactParam, ContactResult> for ContactAgent {
    fn process(
        &self,
        _input: ContactParam,
    ) -> impl std::future::Future<Output = Result<ContactResult, AgentError>> + Send {
        async move {
            // TODO: Implement contact data discovery logic

            Err(AgentError::ProcessingError(
                "Contact finding not implemented yet".to_string(),
            ))
        }
    }
}
