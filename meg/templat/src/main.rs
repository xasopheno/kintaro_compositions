use handlebars::{to_json, Handlebars};
#[allow(unused_imports)]
use log::{debug, error, info, log, log_enabled, Level};
use num_rational::Rational64;
use serde::{Deserialize, Serialize};
use serde_json::json;
use serde_json::value::{Map, Value as Json};
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::path::Path;
use walkdir::WalkDir;

#[derive(Serialize)]
pub struct Meg {
    filename: String,
    tag: String,
    pan: String,
    scale: String,
}
const TARGET: &str = "0";
const SCALE: &str = "2.0e13";

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

#[derive(Deserialize)]
struct CoordData {
    sensors: Vec<String>,
    pan: Rational64,
}
type Coords = HashMap<String, CoordData>;

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

    let mut megs: Vec<Meg> = filenames
        .iter()
        .map(|filename| {
            let mut file_stem = Path::new(filename)
                .file_stem()
                .unwrap()
                .to_string_lossy()
                .to_lowercase()
                .split("_")
                .collect::<Vec<&str>>()[0]
                .to_owned();

            file_stem.retain(|s| !r#"meg"#.contains(s));

            let channel_context = get_channel_context(&file_stem, &coords).unwrap();
            Meg {
                filename: filename[3..].to_string(),
                pan: channel_context.pan,
                tag: channel_context.brain_region,
                scale: SCALE.into(),
            }
        })
        .collect();

    megs.sort_by_key(|m| m.filename.clone());

    data.insert("megs".to_string(), to_json(&megs));
    data
}

struct ChannelContext {
    brain_region: String,
    pan: String,
}

fn get_channel_context(filestem: &str, coords: &Coords) -> Option<ChannelContext> {
    for (brain_region, values) in coords.into_iter() {
        for sensor in &values.sensors {
            if filestem.starts_with(sensor) {
                println!(
                    "{filestem}->{sensor}->{brain_region}->{} {}",
                    values.pan.numer(),
                    values.pan.denom()
                );
                return Some(ChannelContext {
                    brain_region: brain_region.clone(),
                    pan: format!("{}/{}", values.pan.numer(), values.pan.denom()),
                });
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
