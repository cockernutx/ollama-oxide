use serde::{Deserialize, Serialize};


/// Request and response structs for each endpoint.
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ListModelsResponse {
    pub models: Vec<ModelInfo>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ModelInfo {
    pub name: String,
    pub size: u64,
    pub modified_at: String,
    pub digest: String,
    pub details: ModelDetails,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ModelDetails {
    pub format: String,
    pub family: String,
    pub families: Option<Vec<String>>,
    pub parameter_size: String,
    pub quantization_level: String,
}

#[derive(Serialize, Debug, Default)]
pub struct ShowModelRequest {
    pub name: String,
}

#[derive(Serialize, Debug, Default)]
pub struct PullModelRequest {
    pub name: String,
}

#[derive(Deserialize, Debug, Default)]
pub struct PullResponse {
    pub status: Option<String>,
    pub digest: Option<String>,
    pub total: Option<u64>,
    pub completed: Option<u64>,
}

#[derive(Serialize, Debug, Default)]
pub struct GenerateRequest {
    pub model: String,
    pub prompt: String,
    pub stream: Option<bool>,
    pub format: Option<String>,
    pub options: Option<GenerateOptions>,
}

#[derive(Serialize, Debug, Default)]
pub struct GenerateOptions {
    pub temperature: Option<f64>,
    pub top_p: Option<f64>,
    pub max_tokens: Option<u32>,
    pub stop: Option<Vec<String>>,
}

#[derive(Deserialize, Debug, Default)]
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

#[derive(Serialize, Debug, Default)]
pub struct ChatRequest {
    pub model: String,
    pub messages: Vec<ChatMessage>,
    pub stream: Option<bool>,
    pub format: Option<String>,
    pub options: Option<GenerateOptions>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
    pub images: Option<Vec<String>>,
    pub tool_calls: Option<Vec<ToolCall>>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ToolCall {
    pub function: FunctionCall,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct FunctionCall {
    pub name: String,
    pub arguments: serde_json::Value,
}

#[derive(Deserialize, Debug, Default)]
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

#[derive(Serialize, Debug, Default)]
pub struct CreateModelRequest {
    pub model: String,
    pub from: Option<String>,
    pub files: Option<serde_json::Value>,
    pub adapters: Option<serde_json::Value>,
    pub template: Option<String>,
    pub license: Option<String>,
    pub system: Option<String>,
    pub parameters: Option<serde_json::Value>,
    pub messages: Option<Vec<ChatMessage>>,
    pub stream: Option<bool>,
    pub quantize: Option<String>,
}

#[derive(Deserialize, Debug, Default)]
pub struct CreateResponse {
    pub status: Option<String>,
    pub digest: Option<String>,
    pub total: Option<u64>,
    pub completed: Option<u64>,
}

#[derive(Serialize, Debug, Default)]
pub struct PushModelRequest {
    pub name: String,
}

#[derive(Deserialize, Debug, Default)]
pub struct PushResponse {
    pub status: Option<String>,
    pub digest: Option<String>,
    pub total: Option<u64>,
    pub completed: Option<u64>,
}

#[derive(Serialize, Debug, Default)]
pub struct DeleteModelRequest {
    pub name: String,
}

#[derive(Serialize, Debug)]
#[serde(untagged)]
pub enum EmbedInput {
    Single(String),
    Multiple(Vec<String>),
}

impl Default for EmbedInput {
    fn default() -> Self {
        Self::Single("".to_string())
    }
}

#[derive(Serialize, Debug, Default)]
pub struct EmbedRequest {
    pub model: String,
    pub input: EmbedInput,
    pub truncate: Option<bool>,
    pub options: Option<GenerateOptions>,
    pub keep_alive: Option<String>,
}

#[derive(Deserialize, Debug, Default)]
pub struct EmbedResponse {
    pub model: String,
    pub embeddings: Vec<Vec<f32>>,
    pub total_duration: u64,
    pub load_duration: u64,
    pub prompt_eval_count: u32,
}

#[derive(Deserialize, Debug, Default)]
pub struct ListRunningModelsResponse {
    pub models: Vec<RunningModelInfo>,
}

#[derive(Deserialize, Debug, Default)]
pub struct RunningModelInfo {
    pub name: String,
    pub model: String,
    pub size: u64,
    pub digest: String,
    pub details: ModelDetails,
    pub expires_at: String,
    pub size_vram: u64,
}

#[derive(Deserialize, Debug, Default)]
pub struct VersionResponse {
    pub version: String,
}