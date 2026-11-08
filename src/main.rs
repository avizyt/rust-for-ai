mod analyzer;
mod models;
mod reader;

use reader::analyze_file;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let report = analyze_file("./data/inference_logs.jsonl")?;

    println!("\n===== AI WORKLOAD REPORT =====");

    println!("Total requests: {}", report.total_requests);

    println!("Total prompt tokens: {}", report.total_prompt_tokens);

    println!(
        "Total completion tokens: {}",
        report.total_completion_tokens
    );

    println!(
        "Total tokens: {}",
        report.total_prompt_tokens + report.total_completion_tokens
    );

    println!(
        "Average latency: {:.2} ms",
        report.total_latency_ms as f64 / report.total_requests.max(1) as f64
    );

    println!("\n===== MODEL STATISTICS =====");

    for (model, stats) in &report.models {
        println!("\nModel: {model}");
        println!("Requests: {}", stats.request_count);
        println!("Total tokens: {}", stats.total_tokens());

        println!("Average latency: {:.2} ms", stats.average_latency_ms());
    }

    Ok(())
}
