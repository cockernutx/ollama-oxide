use reqwest::Client;
use futures::{Stream, TryStreamExt};
use error::OllamaError;
use models::*;
use tracing::debug;

pub mod error;
pub mod models;


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
        let text = response.text().await?;
        debug!("ollama response: {}", text);
        let response_body: GenerateResponse = serde_json::from_str(&text)?;
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

/// Generates embeddings from a model.
pub async fn generate_embeddings(&self, request: EmbedRequest) -> Result<EmbedResponse, OllamaError> {
    let url = format!("{}/api/embed", self.base_url);
    let response = self.client.post(&url).json(&request).send().await?;

    if response.status().is_success() {
        let response_body: EmbedResponse = response.json().await?;
        Ok(response_body)
    } else {
        let status = response.status();
        let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
        Err(OllamaError::ApiError(format!("Status: {}, Error: {}", status, error_text)))
    }
}
pub async fn generate_multiple_embeddings(
    &self,
    model: String,
    inputs: Vec<String>,
    truncate: Option<bool>,
    options: Option<GenerateOptions>,
    keep_alive: Option<String>,
) -> Result<EmbedResponse, OllamaError> {
    let url = format!("{}/api/embed", self.base_url);
    let request = EmbedRequest {
        model,
        input: EmbedInput::Multiple(inputs),
        truncate,
        options,
        keep_alive,
    };

    let response = self.client.post(&url).json(&request).send().await?;

    if response.status().is_success() {
        let response_body: EmbedResponse = response.json().await?;
        Ok(response_body)
    } else {
        let status = response.status();
        let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
        Err(OllamaError::ApiError(format!("Status: {}, Error: {}", status, error_text)))
    }
}
/// Lists running models.
pub async fn list_running_models(&self) -> Result<Vec<RunningModelInfo>, OllamaError> {
    let url = format!("{}/api/ps", self.base_url);
    let response = self.client.get(&url).send().await?;

    if response.status().is_success() {
        let response_body: ListRunningModelsResponse = response.json().await?;
        Ok(response_body.models)
    } else {
        let status = response.status();
        let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
        Err(OllamaError::ApiError(format!("Status: {}, Error: {}", status, error_text)))
    }
}

/// Retrieves the Ollama version.
pub async fn get_version(&self) -> Result<String, OllamaError> {
    let url = format!("{}/api/version", self.base_url);
    let response = self.client.get(&url).send().await?;

    if response.status().is_success() {
        let response_body: VersionResponse = response.json().await?;
        Ok(response_body.version)
    } else {
        let status = response.status();
        let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
        Err(OllamaError::ApiError(format!("Status: {}, Error: {}", status, error_text)))
    }
}
}
