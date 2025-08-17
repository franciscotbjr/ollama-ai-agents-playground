use serde::{Deserialize, Serialize};

use crate::agent::Intent;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ClassificationResult {
    pub intent: Intent,
    pub params: serde_json::Value,
}

impl ClassificationResult {
    pub fn new(intent: Intent, params: serde_json::Value) -> Self {
        Self { intent, params }
    }

    pub fn from_json_str(json_str: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json_str)
    }

    pub fn to_json_string(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }
}
