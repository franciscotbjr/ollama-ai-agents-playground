use crate::config::Config;
use crate::infra::http::HttpClient;
use crate::infra::ollama::{
    OllamaChatRequest, OllamaCreateRequest, OllamaCreateResponse, OllamaResponse,
};

pub struct OllamaClient {
    http_client: HttpClient,
}

impl OllamaClient {
    pub fn new() -> Self {
        Self {
            http_client: HttpClient::new(Config::get().ollama.api.url.clone()),
        }
    }

    pub async fn send_chat_request(
        &self,
        body: &str,
    ) -> Result<OllamaResponse, Box<dyn std::error::Error>> {
        let ollama_request =
            OllamaChatRequest::new(Config::get().ollama.api.model.clone(), body.to_string());

        let json_request = serde_json::to_string(&ollama_request);

        match json_request {
            Ok(request_body) => {
                let response = self
                    .http_client
                    .send_request::<OllamaResponse>(request_body.as_str())
                    .await?;

                if response.success {
                    response
                        .data
                        .ok_or_else(|| "No data received from Ollama API".into())
                } else {
                    let error_msg = response
                        .error
                        .map(|e| format!("{}: {}", e.error, e.message))
                        .unwrap_or_else(|| "Unknown error occurred".to_string());
                    Err(error_msg.into())
                }
            }
            Err(e) => Err(e.to_string().into()),
        }
    }

    pub async fn send_message(
        &self,
        prompt: &str,
    ) -> Result<OllamaResponse, Box<dyn std::error::Error>> {
        let request_body = format!(r#"{}"#, prompt.replace('"', "\\\""));
        self.send_chat_request(&request_body).await
    }

    pub async fn create_assistant(
        &self,
        system: String,
        name: String,
    ) -> Result<OllamaCreateResponse, Box<dyn std::error::Error>> {
        let create_request = OllamaCreateRequest::new(
            Config::get().ollama.api.model.clone(), 
            system.clone(), 
            name.clone());

        let json_request = serde_json::to_string(&create_request);

        match json_request {
            Ok(request_body) => {
                let response = self
                    .http_client
                    .send_request::<OllamaCreateResponse>(request_body.as_str())
                    .await?;
                if response.success {
                    response
                        .data
                        .ok_or_else(|| "No data received from Ollama API".into())
                } else {
                    let error_msg = response
                        .error
                        .map(|e| format!("{}: {}", e.error, e.message))
                        .unwrap_or_else(|| "Unknown error occurred".to_string());
                    Err(error_msg.into())
                }
            }
            Err(e) => Err(e.to_string().into()),
        }

    }
}
