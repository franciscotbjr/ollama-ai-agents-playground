use crate::agents::agent_param::AgentParam;

#[derive(Debug, Clone)]
pub struct CreateParam {
    assistant_to: String,
    name: String,
}

impl CreateParam {
    pub fn new(assistant_to: String, name: String) -> Self {
        Self { assistant_to, name }
    }

    pub fn assistant_to(&self) -> &str {
        &self.assistant_to
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

impl AgentParam for CreateParam {}
