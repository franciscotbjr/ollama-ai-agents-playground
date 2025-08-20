use serde::{Deserialize, Serialize};

use crate::agent::AgentResult;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ContactResult {}

impl ContactResult {
    pub fn new() -> Self {
        Self {}
    }
}

impl AgentResult for ContactResult {}
