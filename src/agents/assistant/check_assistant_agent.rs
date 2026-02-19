use crate::{
    agents::{Agent, AgentError, assistant::CheckResult},
    infra::assistant_ollama_client::AssistantOllamaClient,
};

use super::check_param::CheckParam;

#[derive(Debug, Default)]
pub struct CheckAssistantAgent {}

impl Agent<CheckParam, CheckResult> for CheckAssistantAgent {
    fn process(
        &self,
        input: CheckParam,
    ) -> impl std::future::Future<Output = Result<CheckResult, crate::agents::AgentError>> + Send
    {
        async move {
            let check_result = AssistantOllamaClient::new()
                .check_model_exists(input.name())
                .await;

            match check_result {
                Ok(check) => Ok(CheckResult::new(check.exists)),
                Err(e) => Err(AgentError::ParseError(format!("Failed to check: {e}"))),
            }
        }
    }
}
