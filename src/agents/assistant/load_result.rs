use serde::{Deserialize, Serialize};

use crate::agents::AgentResult;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LoadResult {
    #[serde(default)]
    pub success: bool,
}

impl LoadResult {
    pub fn new(success: bool) -> Self {
        Self { success }
    }
}

impl AgentResult for LoadResult {}
