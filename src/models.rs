use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct InferenceRecord {
    pub request_id: String,
    pub model: String,
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub latency_ms: u64,
}

#[derive(Debug, Default)]
pub struct ModelStats {
    pub request_count: u64,
    pub total_prompt_tokens: u64,
    pub total_completion_tokens: u64,
    pub total_latency_ms: u64,
}

impl ModelStats {
    pub fn total_tokens(&self) -> u64 {
        self.total_prompt_tokens + self.total_completion_tokens
    }

    pub fn average_latency_ms(&self) -> f64 {
        if self.request_count == 0 {
            return 0.0;
        }

        self.total_latency_ms as f64 / self.request_count as f64
    }
}
