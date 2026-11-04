use rust_lab::{InferenceRequest, OllamaBackend, VllmBackend, inspect_backend, process_request};

fn main() {
    let request = InferenceRequest::new(
        "llama-3.2-1b".to_string(),
        "Explain attention".to_string(),
        256,
        Some(0.7),
    )
    .expect("Invalid inference request");

    let ollama = OllamaBackend::new("llama3.2:1b".to_string());

    let vllm = VllmBackend::new("http://localhost:8000".to_string());

    inspect_backend(&ollama);
    inspect_backend(&vllm);

    process_request(&ollama, &request);
    process_request(&vllm, &request);
}
