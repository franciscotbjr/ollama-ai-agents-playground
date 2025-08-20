use serde::{Deserialize, Serialize};

use crate::agent::AgentResult;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EmailResult {}

impl EmailResult {
    pub fn new() -> Self {
        Self {}
    }
}

impl AgentResult for EmailResult {}
