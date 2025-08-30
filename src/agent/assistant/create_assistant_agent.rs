use crate::{agent::{agent::AgentParam, assistant::{create_result, CreateResult}, Agent, AgentError}, infra::ollama::OllamaClient};

pub struct CreateAssistantAgent {}

impl CreateAssistantAgent {
    pub fn new() -> Self {
        Self {}
    }
}

pub struct CreateParam {
    system: String, 
    name: String,
}

impl CreateParam {
    pub fn new(system: String, name: String) -> Self {
        Self { 
            system,
            name
        }
    }
}

impl AgentParam for CreateParam {}

impl Agent<CreateParam, CreateResult> for  CreateAssistantAgent {
    fn process(
        &self,
        input: CreateParam, 
    ) -> impl std::future::Future<Output = Result<CreateResult, AgentError>> + Send {
        async move {
            let system_prompt = build_system_prompt(&input.name.clone());

            let result = 
                OllamaClient::new().create_assistant(&system_prompt.clone(), &input.name.clone()).await;

            match result {
                Ok(create_result) => {
                    let success_messages: Vec<bool> = create_result.messages.iter().map(|m| m.status.eq_ignore_ascii_case("success")).collect();
                    let has_success = success_messages.iter().any(|&success| success);
                    Ok(CreateResult::new(has_success))
                }
                Err(e) => Err(AgentError::ParseError(format!(
                    "Model creation failed: {}",
                    e
                ))),
            }
        }
    }
}

fn build_system_prompt(input: &str) -> String {
    String::new()
}