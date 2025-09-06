use serde::{Deserialize, Serialize};

use crate::agents::AgentResult;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ContactResult {}

impl ContactResult {
    pub fn new() -> Self {
        Self {}
    }
}

impl AgentResult for ContactResult {}
