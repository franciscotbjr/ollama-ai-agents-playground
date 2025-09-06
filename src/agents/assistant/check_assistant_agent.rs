use crate::{
    agents::{
        Agent, AgentError,
        agent::AgentParam,
        assistant::{CheckResult},
    },
    infra::ollama::{OllamaClient},
};

#[derive(Debug, Default)]
pub struct CheckAssistantAgent {}

#[derive(Debug, Clone)]
pub struct CheckParam {
    name: String,
}

impl CheckParam {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}

impl AgentParam for CheckParam {}

impl Agent<CheckParam, CheckResult> for CheckAssistantAgent {
    fn process(
        &self,
        input: CheckParam,
    ) -> impl std::future::Future<Output = Result<CheckResult, crate::agents::AgentError>> + Send
    {
        async move {
            let check_result = OllamaClient::new().check_model_exists(&input.name).await;

            match check_result {
                Ok(check) => Ok(CheckResult::new(check.exists)),
                Err(e) => Err(AgentError::ParseError(format!("Failed to check: {e}"))),
            }
        }
    }
}
