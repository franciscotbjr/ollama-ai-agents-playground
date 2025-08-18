use crate::infra::ollama::OllamaResponse;

pub struct OllamaClient;

impl OllamaClient {
    pub async fn send_message(_prompt: &str) -> OllamaResponse {
        // TODO: Implement actual Ollama API call
        // For now, return a dummy response to allow compilation
        unimplemented!("OllamaClient::send_message not yet implemented")
    }
}