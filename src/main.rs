fn normalized_model_name(model: &str) -> String {
    model.trim().to_lowercase()
}

fn is_supported_model(model: &str) -> bool {
    if model.starts_with("llama-") {
        true
    } else {
        false
    }
}

fn append_optimized_suffix(model: &mut String) {
    model.push_str("-optmized");
}

fn longest_model_name<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() >= b.len() { a } else { b }
}

fn main() {
    let model_1 = String::from(" Llama-3.2-1B ");
    let model_2 = String::from(" Gemini4-instruct-4B ");

    let mut norm_model_1 = normalized_model_name(&model_1);
    let mut norm_model_2 = normalized_model_name(&model_2);

    append_optimized_suffix(&mut norm_model_1);
    append_optimized_suffix(&mut norm_model_2);

    let longest_model = longest_model_name(&norm_model_1, &norm_model_2);

    println!("Normalized Model 1: {norm_model_1}");
    println!("Normalized Model 2: {norm_model_2}");

    println!("Longest Model: {longest_model}");

    if is_supported_model(&norm_model_1) == true {
        println!("Supported Model: {norm_model_1}");
    } else if is_supported_model(&norm_model_2) == true {
        println!("Supported Model: {norm_model_2}");
    }
}
