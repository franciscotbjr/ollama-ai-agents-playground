use super::agent_prompt::AgentPrompt;

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
        AgentPrompt::new(self.content.unwrap())
    }
}
