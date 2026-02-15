use serde::{Deserialize, Serialize};

use crate::agents::AgentResult;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct CheckResult {
    #[serde(default)]
    pub exists: bool,
}

impl CheckResult {
    pub fn new(exists: bool) -> Self {
        Self { exists }
    }
}

impl AgentResult for CheckResult {}
