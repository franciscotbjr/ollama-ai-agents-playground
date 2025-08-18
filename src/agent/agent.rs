use std::error::Error;

use crate::agent::ClassificationResult;

#[derive(Debug)]
pub enum AgentError {
    ProcessingError(String),
    NetworkError(String),
    ParseError(String),
}

impl std::fmt::Display for AgentError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AgentError::ProcessingError(msg) => write!(f, "Processing error: {}", msg),
            AgentError::NetworkError(msg) => write!(f, "Network error: {}", msg),
            AgentError::ParseError(msg) => write!(f, "Parse error: {}", msg),
        }
    }
}

impl Error for AgentError {}

pub trait Agent {
    fn process(
        &self,
        input: &str,
    ) -> impl std::future::Future<Output = Result<ClassificationResult, AgentError>> + Send;
}
