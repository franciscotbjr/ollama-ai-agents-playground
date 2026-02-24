use std::fmt;

/// Errors that can occur during workflow execution.
#[derive(Debug)]
pub enum WorkflowError {
    /// A required step in the workflow could not be executed.
    StepFailed(String),
    /// The workflow reached an invalid state.
    InvalidState(String),
    /// Communication with an external service (e.g. an agent) failed.
    AgentError(String),
    /// The workflow definition is malformed or incomplete.
    ConfigurationError(String),
}

impl fmt::Display for WorkflowError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::StepFailed(msg) => write!(f, "Step failed: {msg}"),
            Self::InvalidState(msg) => write!(f, "Invalid state: {msg}"),
            Self::AgentError(msg) => write!(f, "Agent error: {msg}"),
            Self::ConfigurationError(msg) => write!(f, "Configuration error: {msg}"),
        }
    }
}

impl std::error::Error for WorkflowError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_step_failed_display() {
        let e = WorkflowError::StepFailed("send_email failed".to_string());
        assert_eq!(e.to_string(), "Step failed: send_email failed");
    }

    #[test]
    fn test_invalid_state_display() {
        let e = WorkflowError::InvalidState("no active step".to_string());
        assert_eq!(e.to_string(), "Invalid state: no active step");
    }

    #[test]
    fn test_agent_error_display() {
        let e = WorkflowError::AgentError("classifier unreachable".to_string());
        assert_eq!(e.to_string(), "Agent error: classifier unreachable");
    }

    #[test]
    fn test_configuration_error_display() {
        let e = WorkflowError::ConfigurationError("missing step definition".to_string());
        assert_eq!(e.to_string(), "Configuration error: missing step definition");
    }

    #[test]
    fn test_workflow_error_implements_std_error() {
        fn accepts_error<E: std::error::Error>(_e: E) {}
        accepts_error(WorkflowError::StepFailed("test".to_string()));
    }

    #[test]
    fn test_workflow_error_debug() {
        let e = WorkflowError::StepFailed("debug test".to_string());
        let debug = format!("{e:?}");
        assert!(debug.contains("StepFailed"));
    }
}
