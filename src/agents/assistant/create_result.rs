use serde::{Deserialize, Serialize};

use crate::agents::AgentResult;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateResult {
    #[serde(default)]
    pub success: bool,
}

impl CreateResult {
    pub fn new(success: bool) -> Self {
        Self { success }
    }
}

impl AgentResult for CreateResult {}
