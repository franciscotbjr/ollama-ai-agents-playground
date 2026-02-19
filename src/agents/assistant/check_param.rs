use crate::agents::agent_param::AgentParam;

#[derive(Debug, Clone)]
pub struct CheckParam {
    name: String,
}

impl CheckParam {
    pub fn new(name: String) -> Self {
        Self { name }
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

impl AgentParam for CheckParam {}
