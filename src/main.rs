// use rust_lab::{
//     EmbeddingBackend, InferenceBackend, InferenceRequest, OllamaBackend, VllmBackend,
//     create_ollama_backend, inspect_backend, process_all_text_backends, process_request,
// };
//
// fn main() {
//     // Create inference request
//     let request = InferenceRequest::new(
//         "llama3.2:1b".to_string(),
//         "Explain how transformer models work".to_string(),
//         256,
//         Some(0.7),
//     )
//     .expect("Invalid inference request");
//
//     // ======================================
//     // Static Dispatch
//     // ======================================
//
//     let ollama = OllamaBackend;
//     let vllm = VllmBackend;
//
//     inspect_backend(&ollama);
//     inspect_backend(&vllm);
//
//     let ollama_response = process_request(&ollama, &request);
//     println!("Ollama response: {}", ollama_response);
//
//     // ======================================
//     // Associated Types
//     // ======================================
//
//     let embedding_backend = EmbeddingBackend;
//
//     let embedding: Vec<f32> = process_request(&embedding_backend, &request);
//
//     println!("Embedding: {:?}", embedding);
//
//     // ======================================
//     // impl Trait
//     // ======================================
//
//     let factory_backend = create_ollama_backend();
//
//     println!("Factory backend: {}", factory_backend.name());
//
//     // ======================================
//     // Dynamic Dispatch
//     // ======================================
//
//     let backends: Vec<Box<dyn InferenceBackend<Output = String>>> =
//         vec![Box::new(OllamaBackend), Box::new(VllmBackend)];
//
//     process_all_text_backends(&backends, &request);
// }

use std::fs::File;

use rust_lab::{
    InferenceRecord, calculate_total_latency, count_requests_by_model, count_slow_requests,
    find_slow_requests, total_tokens_by_model,
};

fn open_log_file(path: &str) -> Result<File, std::io::Error> {
    let file = File::open(path)?;
    Ok(file)
}

fn main() {
    let records = vec![
        InferenceRecord {
            request_id: "req-001".to_string(),
            model: "llama3.2".to_string(),
            prompt_tokens: 120,
            completion_tokens: 250,
            latency_ms: 420,
        },
        InferenceRecord {
            request_id: "req-002".to_string(),
            model: "mistral".to_string(),
            prompt_tokens: 200,
            completion_tokens: 300,
            latency_ms: 650,
        },
        InferenceRecord {
            request_id: "req-003".to_string(),
            model: "llama3.2".to_string(),
            prompt_tokens: 150,
            completion_tokens: 180,
            latency_ms: 380,
        },
    ];

    for record in &records {
        println!(
            "{} | {} | {} ms",
            record.request_id, record.model, record.latency_ms
        );
    }

    for item in count_requests_by_model(&records) {
        println!("{:?}", item);
    }

    let result = find_slow_requests(&records, 400);

    println!("{:?}", result);

    let total_latency = calculate_total_latency(&records);
    println!("{}", total_latency);

    let slow_rqst = count_slow_requests(&records, 200);
    println!("{}", slow_rqst);

    println!("{:?}", total_tokens_by_model(&records));

    let result = open_log_file("./data/inference_logs.jsonl");

    match result {
        Ok(_file) => {
            println!("File opened successfully");
        }

        Err(error) => {
            eprintln!("Failed to open file: {}", error);
        }
    }
}
