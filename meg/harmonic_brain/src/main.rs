mod data;
mod yin;
use colored::*;
use rayon::prelude::*;
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};
use walkdir::WalkDir;
use yin::{Analyze, DetectionResult};

const SAMPLE_RATE: f32 = 1024.0;
const PROBABILITY_THRESHOLD: f32 = 0.3;

#[derive(Clone)]
struct Score {
    count: usize,
    total: usize,
    average_freq: f32,
}

fn main() {
    println!("Running...");
    let pulses = things();
    let input_dir = "../wi1lmhbs";
    let files: Vec<String> = WalkDir::new(input_dir)
        .into_iter()
        .map(|entry| {
            let entry = entry.unwrap().path().display().to_string();
            if entry.ends_with("0.csv") {
                Some(entry)
            } else {
                None
            }
        })
        .collect::<Vec<Option<String>>>()
        .into_iter()
        .filter(|x| x.is_some())
        .map(|x| x.unwrap())
        .collect();

    let results = Arc::new(Mutex::new(HashMap::new()));

    files.par_iter().for_each(|file| {
        let result = process(file);
        results.lock().unwrap().insert(file, result);
    });

    let mut score: Vec<(String, Score)> = results
        .lock()
        .unwrap()
        .to_owned()
        .into_iter()
        .map(|(filename, score)| {
            (
                filename.to_string(),
                Score {
                    count: score.0,
                    total: score.1,
                    average_freq: score.2,
                },
            )
        })
        .collect();

    score.sort_unstable_by_key(|s| s.0.clone());

    score.iter().for_each(|x| {
        let filename = x.0.to_owned();
        let score = x.1.to_owned();
        let percent = score.count as f32 / score.total as f32 * 100.0;
        let percent_str = format!("{}%", percent);
        let is_pulse = pulses.contains(&filename.as_str());
        println!(
            "{}: {}/{} | {} __ {}",
            if is_pulse {
                filename.blue()
            } else {
                filename.black()
            },
            score.count,
            score.total,
            if percent < 15.0 {
                percent_str.red()
            } else {
                percent_str.green()
            },
            score.average_freq
        );
    })
}

fn process(filename: &str) -> (usize, usize, f32) {
    let csv = data::CsvData::get_data(filename.into()).unwrap();
    let input = csv[0].data.chunks(SAMPLE_RATE as usize);
    let mut count = 0;
    let mut total_count = 0;
    let mut total_freq = 0.0;
    input.for_each(|chunk| {
        let detection_result: DetectionResult =
            chunk.to_vec().analyze(SAMPLE_RATE, PROBABILITY_THRESHOLD);

        total_freq += detection_result.frequency;

        if detection_result.probability > 10.0 {
            count += 1;
        }
        total_count += 1;
    });
    return (count, total_count, total_freq / total_count as f32);
}
fn things() -> Vec<&'static str> {
    vec![
        "../wi1lmhbs/meg0123_0.csv",
        "../wi1lmhbs/meg0142_0.csv",
        "../wi1lmhbs/meg0312_0.csv",
        "../wi1lmhbs/meg0313_0.csv",
        "../wi1lmhbs/meg0322_0.csv",
        "../wi1lmhbs/meg0342_0.csv",
        "../wi1lmhbs/meg0512_0.csv",
        "../wi1lmhbs/meg0513_0.csv",
        "../wi1lmhbs/meg0522_0.csv",
        "../wi1lmhbs/meg0523_0.csv",
        "../wi1lmhbs/meg0532_0.csv",
        "../wi1lmhbs/meg0533_0.csv",
        "../wi1lmhbs/meg1222_0.csv",
        "../wi1lmhbs/meg1332_0.csv",
        "../wi1lmhbs/meg1333_0.csv",
        "../wi1lmhbs/meg1432_0.csv",
        "../wi1lmhbs/meg1512_0.csv",
        "../wi1lmhbs/meg1513_0.csv",
        "../wi1lmhbs/meg1522_0.csv",
        "../wi1lmhbs/meg1532_0.csv",
        "../wi1lmhbs/meg1533_0.csv",
        "../wi1lmhbs/meg1542_0.csv",
        "../wi1lmhbs/meg1543_0.csv",
        "../wi1lmhbs/meg1643_0.csv",
        "../wi1lmhbs/meg1712_0.csv",
        "../wi1lmhbs/meg1713_0.csv",
        "../wi1lmhbs/meg1722_0.csv",
        "../wi1lmhbs/meg1723_0.csv",
        "../wi1lmhbs/meg1732_0.csv",
        "../wi1lmhbs/meg1733_0.csv",
        "../wi1lmhbs/meg1743_0.csv",
        "../wi1lmhbs/meg1922_0.csv",
        "../wi1lmhbs/meg1933_0.csv",
        "../wi1lmhbs/meg1943_0.csv",
        "../wi1lmhbs/meg2342_0.csv",
        "../wi1lmhbs/meg2432_0.csv",
        "../wi1lmhbs/meg2433_0.csv",
        "../wi1lmhbs/meg2512_0.csv",
        "../wi1lmhbs/meg2522_0.csv",
        "../wi1lmhbs/meg2523_0.csv",
        "../wi1lmhbs/meg2532_0.csv",
        "../wi1lmhbs/meg2612_0.csv",
        "../wi1lmhbs/meg2613_0.csv",
        "../wi1lmhbs/meg2622_0.csv",
        "../wi1lmhbs/meg2623_0.csv",
        "../wi1lmhbs/meg2633_0.csv",
        "../wi1lmhbs/meg2642_0.csv",
        "../wi1lmhbs/meg2643_0.csv",
    ]
}
