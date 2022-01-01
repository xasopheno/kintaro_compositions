mod data;
mod yin;
use rayon::prelude::*;
use yin::{Analyze, DetectionResult};

const SAMPLE_RATE: f32 = 1024.0;
const PROBABILITY_THRESHOLD: f32 = 0.3;

fn main() {
    let files = vec![
        "./src/meg0111_full.csv",
        "./src/meg0112_full.csv",
        "./src/meg0113_full.csv",
        "./src/meg0312_full.csv",
        "./src/meg0512_full.csv",
        "./src/meg1533_full.csv",
    ];

    files.par_iter().for_each(|file| {
        process(file);
    });
}

fn process(filename: &str) {
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
    println!("{}: {} -> {}", filename, count, count > 30);
}
