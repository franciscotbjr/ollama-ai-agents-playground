use crate::{http::{HttpError, HttpResponse}, model::create::{ollama_create_reponse::OllamaCreateStatusMessage, OllamaCreateResponse}};

pub struct HttpClient {
    client: reqwest::Client,
    base_url: String,
    end_point: String,
}

impl HttpClient {
    pub fn new(base_url: String, end_point: String) -> Self {
        Self {
            client: reqwest::Client::new(),
            base_url,
            end_point,
        }
    }

    pub async fn send_request_for_status<T>(
        &self,
        body: &str,
    ) -> Result<HttpResponse<T>, Box<dyn std::error::Error>>
    where
        T: serde::de::DeserializeOwned,
    {
        let url = &self.base_url;
        let end_point = &self.end_point;

        let response = self
            .client
            .post(format!("{url}{end_point}"))
            .header("Content-Type", "application/json")
            .body(body.to_string())
            .send()
            .await?;

        let status = response.status().as_u16();
        if response.status().is_success() {
            Ok(HttpResponse {
                success: true,
                data: None,
                error: None,
                status,
            })
        } else {
            let error_text = response.text().await?;
            Ok(HttpResponse {
                success: false,
                data: None,
                error: Some(HttpError {
                    error: "HTTP Error".to_string(),
                    message: error_text,
                }),
                status,
            })
        }
    }

    pub async fn send_request_for_json_response<T>(
        &self,
        body: &str,
    ) -> Result<HttpResponse<T>, Box<dyn std::error::Error>>
    where
        T: serde::de::DeserializeOwned,
    {
        let url = &self.base_url;
        let end_point = &self.end_point;

        let response = self
            .client
            .post(format!("{url}{end_point}"))
            .header("Content-Type", "application/json")
            .body(body.to_string())
            .send()
            .await?;

        let status = response.status().as_u16();
        if response.status().is_success() {
            let data: T = response.json().await?;
            Ok(HttpResponse {
                success: true,
                data: Some(data),
                error: None,
                status,
            })
        } else {
            let error_text = response.text().await?;
            Ok(HttpResponse {
                success: false,
                data: None,
                error: Some(HttpError {
                    error: "HTTP Error".to_string(),
                    message: error_text,
                }),
                status,
            })
        }
    }

    /// Sends a request expecting NDJSON (Newline Delimited JSON) response format
    /// Specifically designed for Ollama create endpoint which returns multiple JSON objects
    pub async fn send_request_for_ndjson_response(
        &self,
        body: &str,
    ) -> Result<HttpResponse<OllamaCreateResponse>, Box<dyn std::error::Error>> {
        let url = &self.base_url;
        let end_point = &self.end_point;

        let response = self
            .client
            .post(format!("{url}{end_point}"))
            .header("Content-Type", "application/json")
            .body(body.to_string())
            .send()
            .await?;

        let status = response.status().as_u16();
        if response.status().is_success() {
            let text = response.text().await?;
            let mut messages = Vec::new();

            // Parse each line as a separate JSON object
            for line in text.lines() {
                let trimmed = line.trim();
                if !trimmed.is_empty() {
                    match serde_json::from_str::<OllamaCreateStatusMessage>(trimmed) {
                        Ok(status_msg) => messages.push(status_msg),
                        Err(e) => {
                            return Err(
                                format!("Failed to parse NDJSON line '{trimmed}': {e}").into()
                            );
                        }
                    }
                }
            }

            let data = OllamaCreateResponse::new_with_status_messages(messages);
            Ok(HttpResponse {
                success: true,
                data: Some(data),
                error: None,
                status,
            })
        } else {
            let error_text = response.text().await?;
            Ok(HttpResponse {
                success: false,
                data: None,
                error: Some(HttpError {
                    error: "HTTP Error".to_string(),
                    message: error_text,
                }),
                status,
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ndjson_parsing() {
        // Test the NDJSON parsing logic with sample Ollama create response
        let ndjson_text = r#"{"status": "using existing layer sha256:3d0b790534fe4b79525fc3692950408dca41171676ed7e21db57af5c65ef6ab6"}
{"status": "using existing layer sha256:ae370d884f108d16e7cc8fd5259ebc5773a0afa6e078b11f4ed7e39a27e0dfc4"}
{"status": "creating new layer sha256:da6000316a62aec68869d313ee6db5058e48d2f26c9843274d16cbab26c5824a"}
{"status": "writing manifest"}
{"status": "success"}"#;

        let mut messages = Vec::new();

        // Parse each line as a separate JSON object (same logic as send_ndjson_request)
        for line in ndjson_text.lines() {
            let trimmed = line.trim();
            if !trimmed.is_empty() {
                let status_msg: OllamaCreateStatusMessage =
                    serde_json::from_str(trimmed).expect("Should parse NDJSON line successfully");
                messages.push(status_msg);
            }
        }

        assert_eq!(messages.len(), 5);
        assert!(messages[0].status.contains("using existing layer"));
        assert!(messages[1].status.contains("using existing layer"));
        assert!(messages[2].status.contains("creating new layer"));
        assert_eq!(messages[3].status, "writing manifest");
        assert_eq!(messages[4].status, "success");

        let response = OllamaCreateResponse::new_with_status_messages(messages);
        assert_eq!(response.messages.len(), 5);
    }

    #[test]
    fn test_ndjson_parsing_with_empty_lines() {
        let ndjson_text = r#"
{"status": "first message"}

{"status": "second message"}


{"status": "third message"}
"#;

        let mut messages = Vec::new();

        for line in ndjson_text.lines() {
            let trimmed = line.trim();
            if !trimmed.is_empty() {
                let status_msg: OllamaCreateStatusMessage =
                    serde_json::from_str(trimmed).expect("Should parse NDJSON line successfully");
                messages.push(status_msg);
            }
        }

        assert_eq!(messages.len(), 3);
        assert_eq!(messages[0].status, "first message");
        assert_eq!(messages[1].status, "second message");
        assert_eq!(messages[2].status, "third message");
    }

    #[test]
    fn test_ndjson_parsing_invalid_json() {
        let invalid_ndjson = r#"{"status": "valid message"}
invalid json line
{"status": "another valid message"}"#;

        let mut messages = Vec::new();
        let mut parse_errors = Vec::new();

        for line in invalid_ndjson.lines() {
            let trimmed = line.trim();
            if !trimmed.is_empty() {
                match serde_json::from_str::<OllamaCreateStatusMessage>(trimmed) {
                    Ok(status_msg) => messages.push(status_msg),
                    Err(e) => parse_errors.push(e.to_string()),
                }
            }
        }

        assert_eq!(messages.len(), 2);
        assert_eq!(parse_errors.len(), 1);
        assert_eq!(messages[0].status, "valid message");
        assert_eq!(messages[1].status, "another valid message");
    }

    #[test]
    fn test_ndjson_parsing_real_ollama_example() {
        // Exact format from the user's example
        let real_ollama_response = r#"{"status": "using existing layer sha256:3d0b790534fe4b79525fc3692950408dca41171676ed7e21db57af5c65ef6ab6"}
{"status": "using existing layer sha256:ae370d884f108d16e7cc8fd5259ebc5773a0afa6e078b11f4ed7e39a27e0dfc4"}
{"status": "using existing layer sha256:d18a5cc71b84bc4af394a31116bd3932b42241de70c77d2b76d69a314ec8aa12"}
{"status": "creating new layer sha256:da6000316a62aec68869d313ee6db5058e48d2f26c9843274d16cbab26c5824a"}
{"status": "using existing layer sha256:cff3f395ef3756ab63e58b0ad1b32bb6f802905cae1472e6a12034e4246fbbdb"}
{"status": "writing manifest"}
{"status": "success"}"#;

        let mut messages = Vec::new();

        for line in real_ollama_response.lines() {
            let trimmed = line.trim();
            if !trimmed.is_empty() {
                let status_msg: OllamaCreateStatusMessage = serde_json::from_str(trimmed)
                    .expect("Should parse real Ollama NDJSON response");
                messages.push(status_msg);
            }
        }

        assert_eq!(messages.len(), 7);
        assert!(messages[0].status.contains("sha256:3d0b790534fe"));
        assert!(messages[1].status.contains("sha256:ae370d884f10"));
        assert!(messages[2].status.contains("sha256:d18a5cc71b84"));
        assert!(messages[3].status.contains("creating new layer"));
        assert!(messages[4].status.contains("sha256:cff3f395ef37"));
        assert_eq!(messages[5].status, "writing manifest");
        assert_eq!(messages[6].status, "success");

        // Verify the response can be created successfully
        let response = OllamaCreateResponse::new_with_status_messages(messages);
        assert_eq!(response.messages.len(), 7);

        // Test that the last message has "success" status for create_assistant_agent logic
        assert_eq!(response.messages.last().unwrap().status, "success");
    }
}