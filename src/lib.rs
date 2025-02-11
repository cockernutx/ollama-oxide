use reqwest::Client;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use futures::{Stream, TryStreamExt};

/// Custom error type for the Ollama client.
#[derive(Debug, Error)]
pub enum OllamaError {
    #[error("HTTP request failed: {0}")]
    RequestFailed(#[from] reqwest::Error),
    #[error("API returned an error: {0}")]
    ApiError(String),
    #[error("Invalid response format: {0}")]
    InvalidResponseFormat(#[from] serde_json::Error),
    #[error("Timeout while waiting for response")]
    Timeout,
}

/// Client for interacting with the Ollama API.
pub struct OllamaClient {
    client: Client,
    base_url: String,
}

impl OllamaClient {
    /// Creates a new Ollama client.
    pub fn new(base_url: &str) -> Self {
        OllamaClient {
            client: Client::new(),
            base_url: base_url.to_string(),
        }
    }

    /// Lists all locally available models.
    pub async fn list_models(&self) -> Result<Vec<ModelInfo>, OllamaError> {
        let url = format!("{}/api/tags", self.base_url);
        let response = self.client.get(&url).send().await?;

        if response.status().is_success() {
            let response_body: ListModelsResponse = response.json().await?;
            Ok(response_body.models)
        } else {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            Err(OllamaError::ApiError(format!("Status: {}, Error: {}", status, error_text)))
        }
    }

    /// Shows information about a specific model.
    pub async fn show_model(&self, model_name: &str) -> Result<ModelInfo, OllamaError> {
        let url = format!("{}/api/show", self.base_url);
        let request = ShowModelRequest {
            name: model_name.to_string(),
        };

        let response = self.client.post(&url).json(&request).send().await?;

        if response.status().is_success() {
            let response_body: ModelInfo = response.json().await?;
            Ok(response_body)
        } else {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            Err(OllamaError::ApiError(format!("Status: {}, Error: {}", status, error_text)))
        }
    }

    /// Pulls a model from the registry.
    pub async fn pull_model(&self, model_name: &str) -> Result<impl Stream<Item = Result<PullResponse, OllamaError>>, OllamaError> {
        let url = format!("{}/api/pull", self.base_url);
        let request = PullModelRequest {
            name: model_name.to_string(),
        };

        let response = self.client.post(&url).json(&request).send().await?;

        if response.status().is_success() {
            let stream = response
                .bytes_stream()
                .map_err(OllamaError::RequestFailed)
                .and_then(|chunk| async move {
                    serde_json::from_slice::<PullResponse>(&chunk).map_err(OllamaError::from)
                });
            Ok(stream)
        } else {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            Err(OllamaError::ApiError(format!("Status: {}, Error: {}", status, error_text)))
        }
    }

    /// Generates a completion using a model.
    pub async fn generate(&self, request: GenerateRequest) -> Result<GenerateResponse, OllamaError> {
        let url = format!("{}/api/generate", self.base_url);
        let response = self.client.post(&url).json(&request).send().await?;

        if response.status().is_success() {
            let response_body: GenerateResponse = response.json().await?;
            Ok(response_body)
        } else {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            Err(OllamaError::ApiError(format!("Status: {}, Error: {}", status, error_text)))
        }
    }

    /// Chats with a model.
    pub async fn chat(&self, request: ChatRequest) -> Result<impl Stream<Item = Result<ChatResponse, OllamaError>>, OllamaError> {
        let url = format!("{}/api/chat", self.base_url);
        let response = self.client.post(&url).json(&request).send().await?;

        if response.status().is_success() {
            let stream = response
                .bytes_stream()
                .map_err(OllamaError::RequestFailed)
                .and_then(|chunk| async move {
                    serde_json::from_slice::<ChatResponse>(&chunk).map_err(OllamaError::from)
                });
            Ok(stream)
        } else {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            Err(OllamaError::ApiError(format!("Status: {}, Error: {}", status, error_text)))
        }
    }

    /// Creates a new model.
    pub async fn create_model(&self, request: CreateModelRequest) -> Result<impl Stream<Item = Result<CreateResponse, OllamaError>>, OllamaError> {
        let url = format!("{}/api/create", self.base_url);
        let response = self.client.post(&url).json(&request).send().await?;

        if response.status().is_success() {
            let stream = response
                .bytes_stream()
                .map_err(OllamaError::RequestFailed)
                .and_then(|chunk| async move {
                    serde_json::from_slice::<CreateResponse>(&chunk).map_err(OllamaError::from)
                });
            Ok(stream)
        } else {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            Err(OllamaError::ApiError(format!("Status: {}, Error: {}", status, error_text)))
        }
    }

    /// Pushes a model to the registry.
    pub async fn push_model(&self, model_name: &str) -> Result<impl Stream<Item = Result<PushResponse, OllamaError>>, OllamaError> {
        let url = format!("{}/api/push", self.base_url);
        let request = PushModelRequest {
            name: model_name.to_string(),
        };

        let response = self.client.post(&url).json(&request).send().await?;

        if response.status().is_success() {
            let stream = response
                .bytes_stream()
                .map_err(OllamaError::RequestFailed)
                .and_then(|chunk| async move {
                    serde_json::from_slice::<PushResponse>(&chunk).map_err(OllamaError::from)
                });
            Ok(stream)
        } else {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            Err(OllamaError::ApiError(format!("Status: {}, Error: {}", status, error_text)))
        }
    }

    /// Deletes a model.
    pub async fn delete_model(&self, model_name: &str) -> Result<(), OllamaError> {
        let url = format!("{}/api/delete", self.base_url);
        let request = DeleteModelRequest {
            name: model_name.to_string(),
        };

        let response = self.client.delete(&url).json(&request).send().await?;

        if response.status().is_success() {
            Ok(())
        } else {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            Err(OllamaError::ApiError(format!("Status: {}, Error: {}", status, error_text)))
        }
    }
}

/// Request and response structs for each endpoint.
#[derive(Serialize, Deserialize, Debug)]
pub struct ListModelsResponse {
    pub models: Vec<ModelInfo>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ModelInfo {
    pub name: String,
    pub size: u64,
    pub modified_at: String,
}

#[derive(Serialize, Debug)]
pub struct ShowModelRequest {
    pub name: String,
}

#[derive(Serialize, Debug)]
pub struct PullModelRequest {
    pub name: String,
}

#[derive(Deserialize, Debug)]
pub struct PullResponse {
    pub status: String,
    pub digest: Option<String>,
    pub total: Option<u64>,
    pub completed: Option<u64>,
}

#[derive(Serialize, Debug)]
pub struct GenerateRequest {
    pub model: String,
    pub prompt: String,
    pub stream: Option<bool>,
    pub format: Option<String>,
    pub options: Option<GenerateOptions>,
}

#[derive(Serialize, Debug)]
pub struct GenerateOptions {
    pub temperature: Option<f64>,
    pub top_p: Option<f64>,
    pub max_tokens: Option<u32>,
    pub stop: Option<Vec<String>>,
}

#[derive(Deserialize, Debug)]
pub struct GenerateResponse {
    pub model: String,
    pub created_at: String,
    pub response: String,
    pub done: bool,
    pub context: Option<Vec<u32>>,
    pub total_duration: Option<u64>,
    pub load_duration: Option<u64>,
    pub prompt_eval_count: Option<u32>,
    pub prompt_eval_duration: Option<u64>,
    pub eval_count: Option<u32>,
    pub eval_duration: Option<u64>,
}

#[derive(Serialize, Debug)]
pub struct ChatRequest {
    pub model: String,
    pub messages: Vec<ChatMessage>,
    pub stream: Option<bool>,
    pub format: Option<String>,
    pub options: Option<GenerateOptions>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
}

#[derive(Deserialize, Debug)]
pub struct ChatResponse {
    pub model: String,
    pub created_at: String,
    pub message: ChatMessage,
    pub done: bool,
    pub total_duration: Option<u64>,
    pub load_duration: Option<u64>,
    pub prompt_eval_count: Option<u32>,
    pub prompt_eval_duration: Option<u64>,
    pub eval_count: Option<u32>,
    pub eval_duration: Option<u64>,
}

#[derive(Serialize, Debug)]
pub struct CreateModelRequest {
    pub name: String,
    pub modelfile: String,
    pub stream: Option<bool>,
}

#[derive(Deserialize, Debug)]
pub struct CreateResponse {
    pub status: String,
    pub digest: Option<String>,
    pub total: Option<u64>,
    pub completed: Option<u64>,
}

#[derive(Serialize, Debug)]
pub struct PushModelRequest {
    pub name: String,
}

#[derive(Deserialize, Debug)]
pub struct PushResponse {
    pub status: String,
    pub digest: Option<String>,
    pub total: Option<u64>,
    pub completed: Option<u64>,
}

#[derive(Serialize, Debug)]
pub struct DeleteModelRequest {
    pub name: String,
}
