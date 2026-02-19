use std::error::Error;

#[derive(Debug)]
pub enum AgentError {
    ProcessingError(String),
    NetworkError(String),
    ParseError(String),
}

impl std::fmt::Display for AgentError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AgentError::ProcessingError(msg) => write!(f, "Processing error: {msg}"),
            AgentError::NetworkError(msg) => write!(f, "Network error: {msg}"),
            AgentError::ParseError(msg) => write!(f, "Parse error: {msg}"),
        }
    }
}

impl Error for AgentError {}
