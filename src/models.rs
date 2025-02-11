use serde::{Deserialize, Serialize};

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
