pub mod analyzer;
pub mod models;
pub mod reader;
use std::fmt::Debug;

use std::{
    fs::File,
    io::{BufRead, BufReader},
};

// ==========================================
// 1. Request Errors
// ==========================================

#[derive(Debug, PartialEq)]
pub enum RequestError {
    EmptyModel,
    EmptyPrompt,
    InvalidMaxTokens,
    InvalidTemperature,
}

// ==========================================
// 2. Inference Request
// ==========================================

#[derive(Debug)]
pub struct InferenceRequest {
    model: String,
    prompt: String,
    max_tokens: u32,
    temperature: Option<f32>,
}

impl InferenceRequest {
    pub fn new(
        model: String,
        prompt: String,
        max_tokens: u32,
        temperature: Option<f32>,
    ) -> Result<Self, RequestError> {
        if model.trim().is_empty() {
            return Err(RequestError::EmptyModel);
        }

        if prompt.trim().is_empty() {
            return Err(RequestError::EmptyPrompt);
        }

        if max_tokens == 0 {
            return Err(RequestError::InvalidMaxTokens);
        }

        if let Some(value) = temperature {
            if !value.is_finite() {
                return Err(RequestError::InvalidTemperature);
            }

            if !(0.0..=2.0).contains(&value) {
                return Err(RequestError::InvalidTemperature);
            }
        }

        Ok(Self {
            model,
            prompt,
            max_tokens,
            temperature,
        })
    }

    pub fn model(&self) -> &str {
        &self.model
    }

    pub fn prompt(&self) -> &str {
        &self.prompt
    }

    pub fn max_tokens(&self) -> u32 {
        self.max_tokens
    }

    pub fn temperature(&self) -> Option<f32> {
        self.temperature
    }
}

// ==========================================
// 3. Inference Backend Trait
// ==========================================

pub trait InferenceBackend {
    type Output;

    fn name(&self) -> &str;

    fn process(&self, request: &InferenceRequest) -> Self::Output;
}

// ==========================================
// 4. Ollama Backend
// ==========================================

#[derive(Debug)]
pub struct OllamaBackend;

impl InferenceBackend for OllamaBackend {
    type Output = String;

    fn name(&self) -> &str {
        "Ollama"
    }

    fn process(&self, request: &InferenceRequest) -> Self::Output {
        format!(
            "Ollama generated a response using model: {}",
            request.model()
        )
    }
}

// ==========================================
// 5. vLLM Backend
// ==========================================

#[derive(Debug)]
pub struct VllmBackend;

impl InferenceBackend for VllmBackend {
    type Output = String;

    fn name(&self) -> &str {
        "vLLM"
    }

    fn process(&self, request: &InferenceRequest) -> Self::Output {
        format!("vLLM generated a response using model: {}", request.model())
    }
}

// ==========================================
// 6. Embedding Backend
// ==========================================

#[derive(Debug)]
pub struct EmbeddingBackend;

impl InferenceBackend for EmbeddingBackend {
    type Output = Vec<f32>;

    fn name(&self) -> &str {
        "Embedding"
    }

    fn process(&self, _request: &InferenceRequest) -> Self::Output {
        vec![0.12, 0.45, 0.78]
    }
}

// ==========================================
// 7. Static Dispatch
// ==========================================

pub fn inspect_backend<B>(backend: &B)
where
    B: InferenceBackend + Debug,
{
    println!("Backend: {}", backend.name());
    println!("Details: {:?}", backend);
}

// ==========================================
// 8. Generic Processing
// ==========================================

pub fn process_request<B>(backend: &B, request: &InferenceRequest) -> B::Output
where
    B: InferenceBackend,
{
    backend.process(request)
}

// ==========================================
// 9. impl Trait
// ==========================================

pub fn create_ollama_backend() -> impl InferenceBackend<Output = String> + Debug {
    OllamaBackend
}

// ==========================================
// 10. Dynamic Dispatch
// ==========================================

pub fn process_all_text_backends(
    backends: &[Box<dyn InferenceBackend<Output = String>>],
    request: &InferenceRequest,
) {
    for backend in backends {
        let response = backend.process(request);

        println!("[{}] {}", backend.name(), response);
    }
}

use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct InferenceRecord {
    pub request_id: String,
    pub model: String,
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub latency_ms: u64,
}

pub fn count_requests_by_model(records: &[InferenceRecord]) -> HashMap<String, u32> {
    let mut model_count = HashMap::new();

    for record in records {
        let count = model_count.entry(record.model.to_string()).or_insert(0);
        *count += 1;
    }
    model_count
}

pub fn find_slow_requests(records: &[InferenceRecord], threshold_ms: u64) -> Vec<&InferenceRecord> {
    records
        .iter()
        .filter(|record| record.latency_ms > threshold_ms)
        .collect()
}

pub fn calculate_total_latency(records: &[InferenceRecord]) -> u64 {
    records
        .iter()
        .map(|record| record.latency_ms)
        .fold(0u64, |total, latency| total + latency)
}

pub fn count_slow_requests(records: &[InferenceRecord], threshold_ms: u64) -> usize {
    // let is_slow = |record: &[InferenceRecord]| record.latency_ms > threshold_ms;
    records
        .iter()
        .filter(|record| record.latency_ms > threshold_ms)
        .count()
}

pub fn total_tokens_by_model(records: &[InferenceRecord]) -> HashMap<String, u64> {
    let mut map = HashMap::new();

    for record in records {
        let total_tokens = record.prompt_tokens as u64 + record.completion_tokens as u64;

        let count = map.entry(record.model.clone()).or_insert(0);

        *count += total_tokens;
    }
    map
}

pub fn count_log_lines(path: &str) -> Result<usize, std::io::Error> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut count = 0;

    for line in reader.lines() {
        let _line = line?;
        count += 1;
    }

    Ok(count)
}

// ==========================================
// 11. Unit Tests
// ==========================================

#[cfg(test)]
mod tests {
    use super::*;

    fn valid_request() -> InferenceRequest {
        InferenceRequest::new(
            "llama3.2:1b".to_string(),
            "Explain Rust ownership".to_string(),
            256,
            Some(0.7),
        )
        .unwrap()
    }

    #[test]
    fn creates_valid_request() {
        let request = valid_request();

        assert_eq!(request.model(), "llama3.2:1b");
        assert_eq!(request.prompt(), "Explain Rust ownership");
        assert_eq!(request.max_tokens(), 256);
        assert_eq!(request.temperature(), Some(0.7));
    }

    #[test]
    fn rejects_empty_model() {
        let result = InferenceRequest::new(" ".to_string(), "Hello".to_string(), 100, None);

        assert_eq!(result.unwrap_err(), RequestError::EmptyModel);
    }

    #[test]
    fn rejects_empty_prompt() {
        let result = InferenceRequest::new("llama3.2:1b".to_string(), " ".to_string(), 100, None);

        assert_eq!(result.unwrap_err(), RequestError::EmptyPrompt);
    }

    #[test]
    fn rejects_zero_max_tokens() {
        let result = InferenceRequest::new("llama3.2:1b".to_string(), "Hello".to_string(), 0, None);

        assert_eq!(result.unwrap_err(), RequestError::InvalidMaxTokens);
    }

    #[test]
    fn rejects_invalid_temperature() {
        let result = InferenceRequest::new(
            "llama3.2:1b".to_string(),
            "Hello".to_string(),
            100,
            Some(3.0),
        );

        assert_eq!(result.unwrap_err(), RequestError::InvalidTemperature);
    }

    #[test]
    fn accepts_missing_temperature() {
        let result =
            InferenceRequest::new("llama3.2:1b".to_string(), "Hello".to_string(), 100, None);

        assert!(result.is_ok());
    }

    #[test]
    fn ollama_returns_string() {
        let backend = OllamaBackend;
        let request = valid_request();

        let response: String = backend.process(&request);

        assert!(response.contains("llama3.2:1b"));
    }

    #[test]
    fn vllm_returns_string() {
        let backend = VllmBackend;
        let request = valid_request();

        let response: String = backend.process(&request);

        assert!(response.contains("llama3.2:1b"));
    }

    #[test]
    fn embedding_backend_returns_vector() {
        let backend = EmbeddingBackend;
        let request = valid_request();

        let embedding: Vec<f32> = backend.process(&request);

        assert_eq!(embedding.len(), 3);
    }

    #[test]
    fn factory_returns_ollama_backend() {
        let backend = create_ollama_backend();

        assert_eq!(backend.name(), "Ollama");
    }
}
