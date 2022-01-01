mod data;
mod yin;
use rayon::prelude::*;
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};
use yin::{Analyze, DetectionResult};

// Type inference lets us omit an explicit type signature (which
// would be `HashMap<String, String>` in this example).

const SAMPLE_RATE: f32 = 1024.0;
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
        let count = process(file);
        results.lock().unwrap().insert(file, count);
    });

    let mut score: Vec<(String, usize)> = results
        .lock()
        .unwrap()
        .to_owned()
        .into_iter()
        .map(|(filename, score)| (filename.to_string(), score))
        .collect();

    score.sort();

    score.iter().for_each(|x| {
        println!("{}: {}", x.0, x.1);
    })
}

fn process(filename: &str) -> usize {
    let csv = data::CsvData::get_data(filename.into()).unwrap();
    let input = csv[0].data.chunks(SAMPLE_RATE as usize);
    let mut count = 0;
    input.for_each(|chunk| {
        let detection_result: DetectionResult =
            chunk.to_vec().analyze(SAMPLE_RATE, PROBABILITY_THRESHOLD);

        if detection_result.probability > 20.0 {
            count += 1;
        }
    });
    return count;
    // println!("{}: {} -> {}", filename, count, count > 30);
}
