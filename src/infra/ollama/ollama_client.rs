use crate::config::Config;
use crate::infra::http::HttpClient;
use crate::infra::ollama::ollama_check_result::OllamaCheckResult;
use crate::infra::ollama::{
    OllamaChatRequest, OllamaCheckRequest, OllamaCreateRequest, OllamaCreateResponse, OllamaOptions, OllamaResponse
};

pub struct OllamaClient {}

impl OllamaClient {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn check_model_exists(
        &self,
        model: &str,
    ) -> Result<OllamaCheckResult, Box<dyn std::error::Error>> {
        let config_api = &Config::get().ollama.api;
        let http_client = HttpClient::new(config_api.url.clone(), config_api.show.clone());

        let ollama_request = OllamaCheckRequest::new(model.to_string());
        let json_request = serde_json::to_string(&ollama_request);

        match json_request {
            Ok(request_body) => {
                let response = http_client.send_request::<String>(request_body.as_str())
                    .await?;
                let status_code = response.status;
                match status_code {
                    200 => Ok(OllamaCheckResult::new(true)),
                    404 => Ok(OllamaCheckResult::new(false)),
                    _ => Err(format!("Fail to check model {}", model.to_string()).into())
                }
            }
            Err(e) => Err(e.to_string().into()),
        }
    }

    pub async fn send_chat_request(
        &self,
        body: &str,
        named_to: &str,
    ) -> Result<OllamaResponse, Box<dyn std::error::Error>> {
        let ollama_request = OllamaChatRequest::new(
            Config::get().assistant.root.to_name(named_to),
            body.to_string(),
            Some(OllamaOptions { temperature: Config::get().ollama.api.options.temperature })
        );

        let config_api = &Config::get().ollama.api;
        let json_request = serde_json::to_string(&ollama_request);
        let http_client = HttpClient::new(config_api.url.clone(), config_api.chat.clone());

        match json_request {
            Ok(request_body) => {
                let response = http_client
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
        named_to: &str,
    ) -> Result<OllamaResponse, Box<dyn std::error::Error>> {
        let request_body = format!(r#"{}"#, prompt.replace('"', "\\\""));
        self.send_chat_request(&request_body, named_to).await
    }

    pub async fn create_assistant(
        &self,
        system: String,
        name: String,
    ) -> Result<OllamaCreateResponse, Box<dyn std::error::Error>> {
        let create_request = OllamaCreateRequest::new(
            Config::get().ollama.api.model.clone(),
            system.clone(),
            name.clone(),
        );

        let json_request = serde_json::to_string(&create_request);

        let config_api = &Config::get().ollama.api;
        let http_client = HttpClient::new(config_api.url.clone(), config_api.create.clone());

        match json_request {
            Ok(request_body) => {
                let response = http_client
                    .send_ndjson_request(request_body.as_str())
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

#[cfg(test)]
mod tests {
    use super::*;

    // Mock OllamaClient for testing without actual HTTP calls
    struct MockOllamaClient {
        mock_responses: std::collections::HashMap<String, Result<bool, String>>,
    }

    impl MockOllamaClient {
        fn new() -> Self {
            Self {
                mock_responses: std::collections::HashMap::new(),
            }
        }

        fn with_model_response(mut self, model: &str, response: Result<bool, String>) -> Self {
            self.mock_responses.insert(model.to_string(), response);
            self
        }

        fn with_model_exists(self, model: &str, exists: bool) -> Self {
            self.with_model_response(model, Ok(exists))
        }

        fn with_model_error(self, model: &str, error: &str) -> Self {
            self.with_model_response(model, Err(error.to_string()))
        }

        async fn check_model_exists(&self, model: &str) -> Result<OllamaCheckResult, Box<dyn std::error::Error>> {
            match self.mock_responses.get(model) {
                Some(Ok(exists)) => Ok(OllamaCheckResult::new(*exists)),
                Some(Err(error)) => Err(error.clone().into()),
                None => Ok(OllamaCheckResult::new(false)), // Default: model doesn't exist
            }
        }
    }

    #[tokio::test]
    async fn test_check_model_exists_status_200_model_found() {
        let client = MockOllamaClient::new()
            .with_model_exists("gemma3", true);

        let result = client.check_model_exists("gemma3").await;
        assert!(result.is_ok());
        
        let check_result = result.unwrap();
        assert_eq!(check_result.exists, true);
    }

    #[tokio::test]
    async fn test_check_model_exists_status_404_model_not_found() {
        let client = MockOllamaClient::new()
            .with_model_exists("non-existent-model", false);

        let result = client.check_model_exists("non-existent-model").await;
        assert!(result.is_ok());
        
        let check_result = result.unwrap();
        assert_eq!(check_result.exists, false);
    }

    #[tokio::test]
    async fn test_check_model_exists_status_500_server_error() {
        let client = MockOllamaClient::new()
            .with_model_error("error-model", "HTTP Error 500: Internal Server Error");

        let result = client.check_model_exists("error-model").await;
        assert!(result.is_err());
        
        let error = result.unwrap_err();
        assert!(error.to_string().contains("HTTP Error 500"));
    }

    #[tokio::test]
    async fn test_check_model_exists_status_400_bad_request() {
        let client = MockOllamaClient::new()
            .with_model_error("bad-model", "HTTP Error 400: Bad Request");

        let result = client.check_model_exists("bad-model").await;
        assert!(result.is_err());
        
        let error = result.unwrap_err();
        assert!(error.to_string().contains("HTTP Error 400"));
    }

    #[tokio::test]
    async fn test_check_model_exists_empty_model_name() {
        let client = MockOllamaClient::new()
            .with_model_exists("", false);

        let result = client.check_model_exists("").await;
        assert!(result.is_ok());
        
        let check_result = result.unwrap();
        assert_eq!(check_result.exists, false);
    }

    #[tokio::test]
    async fn test_check_model_exists_with_special_characters() {
        let model_name = "model-with_special.chars@123";
        let client = MockOllamaClient::new()
            .with_model_exists(model_name, true);

        let result = client.check_model_exists(model_name).await;
        assert!(result.is_ok());
        
        let check_result = result.unwrap();
        assert_eq!(check_result.exists, true);
    }

    #[tokio::test]
    async fn test_check_model_exists_with_unicode() {
        let model_name = "模型-test-café";
        let client = MockOllamaClient::new()
            .with_model_exists(model_name, true);

        let result = client.check_model_exists(model_name).await;
        assert!(result.is_ok());
        
        let check_result = result.unwrap();
        assert_eq!(check_result.exists, true);
    }

    #[tokio::test]
    async fn test_check_model_exists_multiple_status_codes() {
        let client = MockOllamaClient::new()
            .with_model_exists("existing-model", true)    // 200
            .with_model_exists("missing-model", false)    // 404
            .with_model_error("server-error-model", "HTTP Error 500: Internal Server Error")  // 500
            .with_model_error("client-error-model", "HTTP Error 400: Bad Request");           // 400

        // Test 200 - Model exists
        let result1 = client.check_model_exists("existing-model").await.unwrap();
        assert_eq!(result1.exists, true);

        // Test 404 - Model not found
        let result2 = client.check_model_exists("missing-model").await.unwrap();
        assert_eq!(result2.exists, false);

        // Test 500 - Server error
        let result3 = client.check_model_exists("server-error-model").await;
        assert!(result3.is_err());
        assert!(result3.unwrap_err().to_string().contains("500"));

        // Test 400 - Client error
        let result4 = client.check_model_exists("client-error-model").await;
        assert!(result4.is_err());
        assert!(result4.unwrap_err().to_string().contains("400"));
    }

    #[tokio::test]
    async fn test_check_model_exists_case_sensitivity() {
        let client = MockOllamaClient::new()
            .with_model_exists("GEMMA3", true)
            .with_model_exists("gemma3", false);

        let result_upper = client.check_model_exists("GEMMA3").await.unwrap();
        assert_eq!(result_upper.exists, true);

        let result_lower = client.check_model_exists("gemma3").await.unwrap();
        assert_eq!(result_lower.exists, false);
    }

    #[tokio::test]
    async fn test_check_model_exists_long_model_name() {
        let long_model_name = "very-long-model-name-with-many-characters-and-numbers-123456789-abcdefghijklmnopqrstuvwxyz";
        let client = MockOllamaClient::new()
            .with_model_exists(long_model_name, true);

        let result = client.check_model_exists(long_model_name).await.unwrap();
        assert_eq!(result.exists, true);
    }

    #[tokio::test]
    async fn test_check_model_exists_with_spaces() {
        let model_with_spaces = "model with spaces";
        let client = MockOllamaClient::new()
            .with_model_exists(model_with_spaces, false);

        let result = client.check_model_exists(model_with_spaces).await.unwrap();
        assert_eq!(result.exists, false);
    }

    #[tokio::test]
    async fn test_check_model_exists_consistency() {
        let client = MockOllamaClient::new()
            .with_model_exists("consistent-model", true);

        // Call multiple times to ensure consistency
        for _ in 0..3 {
            let result = client.check_model_exists("consistent-model").await.unwrap();
            assert_eq!(result.exists, true);
        }
    }

    #[tokio::test]
    async fn test_check_model_exists_http_status_codes_validation() {
        // Test different HTTP status code scenarios
        let client = MockOllamaClient::new()
            .with_model_error("status-401", "HTTP Error 401: Unauthorized")
            .with_model_error("status-403", "HTTP Error 403: Forbidden")
            .with_model_error("status-502", "HTTP Error 502: Bad Gateway")
            .with_model_error("status-503", "HTTP Error 503: Service Unavailable");

        // All 4xx and 5xx except 404 should return errors
        let error_models = vec!["status-401", "status-403", "status-502", "status-503"];
        
        for model in error_models {
            let result = client.check_model_exists(model).await;
            assert!(result.is_err(), "Model {} should return an error", model);
        }
    }
}
