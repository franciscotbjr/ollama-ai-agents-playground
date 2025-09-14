use crate::{
    agents::{agent::AgentParam, assistant::LoadResult, Agent, AgentError}, infra::assistant_ollama_client::AssistantOllamaClient,
};

#[derive(Debug, Default)]
pub struct LoadAssistantAgent {}

#[derive(Debug, Clone)]
pub struct LoadParam {
    name: String,
}

impl LoadParam {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}

impl AgentParam for LoadParam {}

impl Agent<LoadParam, LoadResult> for LoadAssistantAgent {
    fn process(
        &self,
        input: LoadParam,
    ) -> impl std::future::Future<Output = Result<LoadResult, crate::agents::AgentError>> + Send
    {
        async move {
            let load_result = AssistantOllamaClient::new()
            .load_model(&input.name)
            .await;
            match load_result {
                Ok(load) => Ok(LoadResult::new(load.done)),
                Err(e) => Err(AgentError::ParseError(format!("Failed to load: {}", e))),
            }
        }
    }
}
