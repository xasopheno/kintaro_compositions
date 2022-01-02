mod data;
mod yin;
use colored::*;
use rayon::prelude::*;
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};
use yin::{Analyze, DetectionResult};

// Type inference lets us omit an explicit type signature (which
// would be `HashMap<String, String>` in this example).

const SAMPLE_RATE: f32 = 512.0;
const PROBABILITY_THRESHOLD: f32 = 0.3;

fn main() {
    let results = Arc::new(Mutex::new(HashMap::new()));

    let files = vec![
        "./src/meg0111_full.csv",
        "./src/meg0112_full.csv",
        "./src/meg0113_full.csv",
        "./src/meg0312_full.csv",
        "./src/meg0512_full.csv",
        "./src/meg1533_full.csv",
    ];

    files.par_iter().for_each(|file| {
        let result = process(file);
        results.lock().unwrap().insert(file, result);
    });

    let mut score: Vec<(String, (usize, usize))> = results
        .lock()
        .unwrap()
        .to_owned()
        .into_iter()
        .map(|(filename, score)| (filename.to_string(), score))
        .collect();

    score.sort();

    score.iter().for_each(|x| {
        let filename = x.0.to_owned();
        let counts = x.1;
        let percent = counts.0 as f32 / counts.1 as f32 * 100.0;
        let percent_str = format!("{}%", percent);
        println!(
            "{}: {}/{} | {}",
            filename,
            counts.0,
            counts.1,
            if percent < 15.0 {
                percent_str.red()
            } else {
                percent_str.green()
            }
        );
    })
}

fn process(filename: &str) -> (usize, usize) {
    let csv = data::CsvData::get_data(filename.into()).unwrap();
    let input = csv[0].data.chunks(SAMPLE_RATE as usize);
    let mut count = 0;
    let mut total_count = 0;
    input.for_each(|chunk| {
        let detection_result: DetectionResult =
            chunk.to_vec().analyze(SAMPLE_RATE, PROBABILITY_THRESHOLD);

        if detection_result.probability > 20.0 {
            count += 1;
        }
        total_count += 1;
    });
    return (count, total_count);
}
