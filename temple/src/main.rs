mod color;
mod instancer;
use kintaro::config::named_colorsets;
use kintaro::error::KintaroError;
use kintaro::renderable::EventStreamConfig;
use kintaro::renderable::GlyphyConfig;
use kintaro::renderable::RenderableConfig;
use kintaro::renderable::ToyConfig;
use kintaro::{
    application::run, vertex::shape::Shape, Config, Error, InstanceMul, RandIndex, RandPosition,
};

use crate::instancer::MegInstancer;

fn main() -> Result<(), KintaroError> {
    println!("Hello, Brain");
    let config = make_config();
    run("./src/template.socool", config)
}

fn renderable_configs() -> Vec<RenderableConfig<'static>> {
    vec![
        RenderableConfig::Toy(ToyConfig {
            shader_path: "src/toy.wgsl",
        }),
        RenderableConfig::EventStreams(EventStreamConfig {
            socool_path: "src/template.socool".to_string(),
            shader_path: "./src/shader.wgsl",
        }),
        RenderableConfig::Glyphy(GlyphyConfig::GlypyTextConfig {
            text: vec![("Feather Golem", "#ff2300")],
            location: (0.4, 0.9),
            scale: 40.0,
        }),
    ]
}

pub fn make_config<'a>() -> Config<'a> {
    let instance_mul = InstanceMul {
        x: 9.0,
        y: 19.0,
        z: 1.0,
        life: 2.0,
        size: 23.0,
        length: 1.0,
    };
    let (cameras, instance_mul) = Config::handle_save(instance_mul);
    Config {
        renderable_configs: renderable_configs(),
        composition_name: "How Many Musicians Does It Take",
        instancer: Box::new(MegInstancer {}),
        instance_mul,
        accumulation: false,
        volume: 0.20,
        window_size: (1920 * 2, 1080 * 2),
        cameras,
        shape: Shape {
            n_vertices: 70,
            n_indices: 70,
            position: Box::new(RandPosition),
            color: Box::new(color::color_map()),
            indices: Box::new(RandIndex),
        },
    }
}
