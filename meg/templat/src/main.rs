#[allow(unused_imports)]
use log::{debug, error, info, log, log_enabled, Level};
use serde::Serialize;
use serde_json::value::{Map, Value as Json};
use std::collections::HashMap;
use std::path::Path;
use tap::prelude::*;
use walkdir::WalkDir;

use std::error::Error;
use std::fs::File;

use handlebars::{to_json, Handlebars};

#[derive(Serialize)]
pub struct Meg {
    filename: String,
    tag: String,
    pan: f32,
    scale: String,
}
const TARGET: &str = "0";
const SCALE: &str = "2.0e12";

fn get_filenames(target: &str) -> Vec<String> {
    let input_dir = "../../data_meg/new_result/ds003703_download/b2scmyvu_rest_01";

    WalkDir::new(input_dir)
        .into_iter()
        .map(|entry| {
            let entry = entry.unwrap().path().display().to_string();
            if entry.ends_with(format!("{}.csv", target).as_str()) {
                Some(entry)
            } else {
                None
            }
        })
        .collect::<Vec<Option<String>>>()
        .into_iter()
        .filter(|x| x.is_some())
        .map(|x| x.unwrap())
        .filter(|x| {
            Path::new(x)
                .file_stem()
                .unwrap()
                .to_string_lossy()
                .to_lowercase()
                .contains("meg")
        })
        .collect()
}

type Coords = HashMap<String, Vec<String>>;

fn get_coordinates() -> Coords {
    let file = std::fs::File::open("./neuromeg.json").unwrap();
    let reader = std::io::BufReader::new(file);
    let coords: Coords = serde_json::from_reader(reader).unwrap();
    coords
}

pub fn make_data() -> Map<String, Json> {
    let filenames = get_filenames(TARGET);
    let coords = get_coordinates();

    let mut data = Map::new();

    // let megs = vec![Meg {
    // filename: "./data/wi1lmhbs/meg0111_0.csv".to_string(),
    // pan: -1.0,
    // scale: SCALE.to_string(),
    // }];

    let mut megs: Vec<Meg> = filenames
        .iter()
        .map(|filename| {
            let mut filestem = Path::new(filename)
                .file_stem()
                .unwrap()
                .to_string_lossy()
                .to_lowercase()
                .split("_")
                .collect::<Vec<&str>>()[0]
                .to_owned();

            filestem.retain(|s| !r#"meg"#.contains(s));

            let tag = get_tag(&filestem, &coords).expect("no location found");
            Meg {
                filename: filename.to_string(),
                pan: 0.0,
                tag,
                scale: SCALE.into(),
            }
        })
        .collect();

    // megs.sort();
    // panic!();

    data.insert("megs".to_string(), to_json(&megs));
    data
}

fn get_tag(filestem: &str, coords: &Coords) -> Option<String> {
    for (key, values) in coords.into_iter() {
        for value in values {
            if filestem.starts_with(value) {
                return Some(key.clone());
            }
        }
    }
    None
}

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();
    let mut handlebars = Handlebars::new();

    let data = make_data();

    handlebars
        .register_template_file("template", "./src/template/template.hbs")
        .unwrap();

    let filename = "target/template.socool";
    let mut output_file = File::create(filename)?;
    handlebars.render_to_write("template", &data, &mut output_file)?;
    println!("{}", handlebars.render("template", &data)?);
    println!("{} generated", filename);
    Ok(())
}
