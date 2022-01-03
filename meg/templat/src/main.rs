#[allow(unused_imports)]
use log::{debug, error, info, log_enabled, Level};
use serde::Serialize;
use serde_json::value::{Map, Value as Json};
use walkdir::WalkDir;

use std::error::Error;
use std::fs::File;

use handlebars::{to_json, Handlebars};

#[derive(Serialize)]
pub struct Meg {
    filename: String,
    pan: f32,
    scale: String,
}

fn get_filename() -> Vec<String> {
    let input_dir = "../../";
    WalkDir::new(input_dir)
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
        .collect()
}

pub fn make_data() -> Map<String, Json> {
    let mut data = Map::new();

    let megs = vec![Meg {
        filename: "./data/wi1lmhbs/meg0111_0.csv".to_string(),
        pan: -1.0,
        scale: "2.0e12".to_string(),
    }];

    data.insert("megs".to_string(), to_json(&megs));
    data
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
