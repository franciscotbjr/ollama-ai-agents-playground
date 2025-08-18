use std::fmt;

#[derive(Clone)]
pub struct ClassifierPrompt {
    content: String,
}

impl ClassifierPrompt {
    #[allow(dead_code)]
    fn new(content: String) -> Self {
        Self { content }
    }

    pub fn content(&self) -> &str {
        &self.content
    }

    pub fn builder() -> ClassifierPromptBuilder {
        ClassifierPromptBuilder::new()
    }
}

impl fmt::Display for ClassifierPrompt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.content)
    }
}

pub struct ClassifierPromptBuilder {
    content: Option<String>,
}

impl ClassifierPromptBuilder {
    pub fn new() -> Self {
        Self { content: None }
    }

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

    pub fn build(self) -> ClassifierPrompt {
        ClassifierPrompt {
            content: self.content.unwrap(),
        }
    }
}
