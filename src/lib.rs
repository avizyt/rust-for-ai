// ==========================================
// 1. Custom Error Type
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
            if !value.is_finite() || !(0.0..=2.0).contains(&value) {
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

    // Accessors

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
    fn name(&self) -> &str;

    fn process(&self, request: &InferenceRequest) -> String;
}

// ==========================================
// 4. Ollama Backend
// ==========================================

pub struct OllamaBackend {
    model: String,
}

impl OllamaBackend {
    pub fn new(model: String) -> Self {
        Self { model }
    }
}

impl InferenceBackend for OllamaBackend {
    fn name(&self) -> &str {
        "Ollama"
    }

    fn process(&self, request: &InferenceRequest) -> String {
        format!(
            "Processing request for {} using Ollama model {}",
            request.model(),
            self.model
        )
    }
}

// ==========================================
// 5. vLLM Backend
// ==========================================

pub struct VllmBackend {
    endpoint: String,
}

impl VllmBackend {
    pub fn new(endpoint: String) -> Self {
        Self { endpoint }
    }
}

impl InferenceBackend for VllmBackend {
    fn name(&self) -> &str {
        "vLLM"
    }

    fn process(&self, request: &InferenceRequest) -> String {
        format!(
            "Processing request for {} using vLLM endpoint {}",
            request.model(),
            self.endpoint
        )
    }
}

// ==========================================
// 6. Generic Function
// ==========================================

pub fn inspect_backend<B: InferenceBackend>(backend: &B) {
    println!("Backend: {}", backend.name());
}

// ==========================================
// 7. Generic Processing Function
// ==========================================

pub fn process_request<B: InferenceBackend>(backend: &B, request: &InferenceRequest) {
    println!("Backend: {}", backend.name());

    let result = backend.process(request);

    println!("Result: {}", result);
}

// ==========================================
// 8. Unit Tests
// ==========================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_request_should_succeed() {
        let result = InferenceRequest::new(
            "llama-3.2-1b".to_string(),
            "Explain attention".to_string(),
            256,
            Some(0.7),
        );

        assert!(result.is_ok());
    }

    #[test]
    fn empty_model_should_fail() {
        let result = InferenceRequest::new(
            " ".to_string(),
            "Explain attention".to_string(),
            256,
            Some(0.7),
        );

        assert_eq!(result.unwrap_err(), RequestError::EmptyModel);
    }

    #[test]
    fn invalid_temperature_should_fail() {
        let result = InferenceRequest::new(
            "llama-3.2-1b".to_string(),
            "Explain attention".to_string(),
            256,
            Some(3.0),
        );

        assert_eq!(result.unwrap_err(), RequestError::InvalidTemperature);
    }
}
