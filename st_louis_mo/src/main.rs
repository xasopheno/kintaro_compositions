mod color;
mod instancer;
use kintaro::config::{named_colorsets, FramePass};
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

fn frame_passes() -> Vec<FramePass> {
    vec![FramePass {
        output_frame: "main",
        renderables: vec![
            RenderableConfig::Toy(ToyConfig {
                shader_path: "src/toy.wgsl",
            }),
            RenderableConfig::EventStreams(EventStreamConfig {
                socool_path: "./src/template.socool".to_string(),
                shader_path: "./src/shader.wgsl",
                instancer: Box::new(MegInstancer {}),
                shape: Shape {
                    n_vertices: 30,
                    n_indices: 30,
                    position: Box::new(RandPosition),
                    color: Box::new(color::color_map()),
                    indices: Box::new(RandIndex),
                },
            }),
            RenderableConfig::Glyphy(GlyphyConfig::GlypyTextConfig {
                text: vec![("St. Louis, MO", "#321145")],
                location: (0.7, 0.9),
                scale: 50.0,
            }),
        ],
    }]
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
        frame_passes: frame_passes(),
        composition_name: "St. Louis, MO",
        instance_mul,
        volume: 0.20,
        window_size: (1920 * 2, 1080 * 2),
        cameras,
    }
}
