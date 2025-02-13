use thiserror::Error;
/// Custom error type for the Ollama client.
#[derive(Debug, Error)]
pub enum OllamaError {
    #[error("HTTP request failed: {0}")]
    RequestFailed(#[from] reqwest::Error),
    #[error("API returned an error: {0}")]
    ApiError(String),
    #[error("Invalid response format: {0}")]
    InvalidResponseFormat(#[from] serde_json::Error),
    #[error("Invalid response format: {0}")]
    InvalidResponse(String),
    #[error("Timeout while waiting for response")]
    Timeout,
}