use serde::Serialize;
use serde_json::value::{Map, Value as Json};

use std::error::Error;
use std::fs::File;

use handlebars::{
    to_json, Context, Handlebars, Helper, JsonRender, Output, RenderContext, RenderError,
};

#[derive(Serialize)]
pub struct Meg {
    filename: String,
    pan: f32,
    scale: String,
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
