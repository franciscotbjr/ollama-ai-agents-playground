use std::fmt;

#[derive(Clone)]
pub struct AgentPrompt {
    content: String,
}

impl AgentPrompt {
    #[allow(dead_code)]
    fn new(content: String) -> Self {
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

#[derive(Default, Debug)]
pub struct AgentPromptBuilder {
    content: Option<String>,
}

impl AgentPromptBuilder {
    pub fn add_instruction(mut self, instruction: &str) -> Self {
        if self.content.is_none() {
            self.content = Some(instruction.to_string());
        } else {
            let mut content = self.content.unwrap();
            content.push_str(instruction);
            self.content = Some(content);
        }
        self
    }

    pub fn build(self) -> AgentPrompt {
        AgentPrompt {
            content: self.content.unwrap(),
        }
    }
}
