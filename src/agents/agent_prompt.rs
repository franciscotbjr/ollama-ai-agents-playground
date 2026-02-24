use std::fmt;

use super::agent_prompt_builder::AgentPromptBuilder;

#[derive(Clone)]
pub struct AgentPrompt {
    content: String,
}

impl AgentPrompt {
    pub fn new(content: String) -> Self {
        Self { content }
    }

    pub fn content(&self) -> &str {
        &self.content
    }

    pub fn builder() -> AgentPromptBuilder {
        AgentPromptBuilder::default()
    }
}

impl fmt::Display for AgentPrompt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.content)
    }
}
