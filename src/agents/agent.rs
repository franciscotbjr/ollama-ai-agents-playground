use crate::agents::AgentResult;

use super::agent_error::AgentError;
use super::agent_param::AgentParam;

pub trait Agent<P: AgentParam, T: AgentResult> {
    fn process(&self, input: P) -> impl std::future::Future<Output = Result<T, AgentError>> + Send;
}
