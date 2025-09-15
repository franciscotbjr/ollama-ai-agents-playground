use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct HttpError {
    pub error: String,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HttpResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<HttpError>,
    pub status: u16,
}
