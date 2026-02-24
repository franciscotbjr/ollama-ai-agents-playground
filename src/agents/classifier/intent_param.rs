use crate::agents::agent_param::AgentParam;

#[derive(Debug)]
pub struct IntentParam {
    input: String,
    assistant: String,
}

impl IntentParam {
    pub fn new(input: String, assistant: String) -> Self {
        Self { input, assistant }
    }

    pub fn input(&self) -> &str {
        &self.input
    }

    pub fn assistant(&self) -> &str {
        &self.assistant
    }
}

impl AgentParam for IntentParam {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_intent_param_new() {
        let param = IntentParam::new("Test input".to_string(), "assistant-name".to_string());
        assert_eq!(param.input(), "Test input");
        assert_eq!(param.assistant(), "assistant-name");
    }

    #[test]
    fn test_intent_param_with_empty_input() {
        let param = IntentParam::new("".to_string(), "assistant".to_string());
        assert_eq!(param.input(), "");
        assert_eq!(param.assistant(), "assistant");
    }

    #[test]
    fn test_intent_param_with_unicode() {
        let unicode_input = "Envie um email para João sobre café naïve 🌍";
        let param = IntentParam::new(unicode_input.to_string(), "assistente".to_string());
        assert_eq!(param.input(), unicode_input);
        assert_eq!(param.assistant(), "assistente");
    }

    #[test]
    fn test_agent_param_trait_implementation() {
        let param = IntentParam::new("test".to_string(), "assistant".to_string());

        fn accepts_agent_param<T: AgentParam>(_param: T) {}
        accepts_agent_param(param);
    }

    #[test]
    fn test_intent_param_debug() {
        let param = IntentParam::new("debug test".to_string(), "debug-assistant".to_string());
        let debug_str = format!("{:?}", param);

        assert!(debug_str.contains("IntentParam"));
    }
}
