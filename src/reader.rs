use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use crate::{analyzer::WorkloadReport, models::InferenceRecord};

pub fn analyze_file(path: &str) -> Result<WorkloadReport, Box<dyn std::error::Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut report = WorkloadReport::default();

    for line in reader.lines() {
        let line = line?;

        if line.trim().is_empty() {
            continue;
        }

        let record: InferenceRecord = serde_json::from_str(&line)?;

        report.process_record(record);
    }

    Ok(report)
}
