use crate::infra::http::{HttpError, HttpResponse};

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

    pub async fn send_request<T>(
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
            .post(format!("{}{}", url, end_point))
            .header("Content-Type", "application/json")
            .body(body.to_string())
            .send()
            .await?;

        if response.status().is_success() {
            let data: T = response.json().await?;
            Ok(HttpResponse {
                success: true,
                data: Some(data),
                error: None,
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
            })
        }
    }
}
