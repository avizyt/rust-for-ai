use crate::models::{InferenceRecord, ModelStats};
use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct WorkloadReport {
    pub total_requests: u64,
    pub total_prompt_tokens: u64,
    pub total_completion_tokens: u64,
    pub total_latency_ms: u64,
    pub models: HashMap<String, ModelStats>,
}

impl WorkloadReport {
    pub fn process_record(&mut self, record: InferenceRecord) {
        self.total_requests += 1;
        self.total_prompt_tokens += record.prompt_tokens as u64;
        self.total_completion_tokens += record.completion_tokens as u64;
        self.total_latency_ms += record.latency_ms;

        let stats = self.models.entry(record.model).or_default();

        stats.request_count += 1;
        stats.total_prompt_tokens += record.prompt_tokens as u64;
        stats.total_completion_tokens += record.completion_tokens as u64;
        stats.total_latency_ms += record.latency_ms;
    }
}
