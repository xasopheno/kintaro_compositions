mod color;
mod color2;
mod instancer;
mod instancer2;
use crate::color::named_colorsets;
use kintaro::config::FramePass;
use kintaro::error::KintaroError;
use kintaro::renderable::EventStreamConfig;
use kintaro::renderable::GlyphyConfig;
use kintaro::renderable::RenderableConfig;
use kintaro::renderable::ToyConfig;
use kintaro::{
    application::run,
    vertex::shape::{RandCircumference, Shape},
    Config, InstanceMul, RandIndex, RandPosition,
};

fn main() -> Result<(), KintaroError> {
    println!("Hello, Brain");
    let config = make_config();
    run(config)
}

fn frame_passes() -> Vec<FramePass> {
    vec![FramePass {
        output_frame: "main",
        renderables: vec![
            RenderableConfig::Toy(ToyConfig {
                shader_path: "src/toy.wgsl",
            }),
            RenderableConfig::EventStreams(EventStreamConfig {
                shader_path: "./src/shader.wgsl",
                instancer: Box::new(crate::instancer::Instancer1 {}),
                shape: Shape {
                    n_vertices: 50,
                    n_indices: 50,
                    position: Box::new(RandPosition),
                    color: Box::new(color::color_map()),
                    indices: Box::new(RandIndex),
                },
            }),
            // RenderableConfig::EventStreams(EventStreamConfig {
            // shader_path: "./src/shader2.wgsl",
            // instancer: Box::new(crate::instancer2::Instancer2 {}),
            // shape: Shape {
            // n_vertices: 80,
            // n_indices: 80,
            // position: Box::new(RandCircumference),
            // color: Box::new(color2::color_map()),
            // indices: Box::new(RandIndex),
            // },
            // }),
            //
            RenderableConfig::Glyphy(GlyphyConfig::GlyphyNamedColorSetConfig {
                text: named_colorsets(),
                location: (0.05, 0.83),
                scale: 35.0,
            }),
            RenderableConfig::Glyphy(GlyphyConfig::GlypyTextConfig {
                text: vec![("Geneva. Wed. Dec 22, 2022", "#3811dd")],
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
        socool_path: "./src/geneva.socool",
        frame_passes: frame_passes(),
        composition_name: "Geneva. Wed. Dec 22, 2022",

        volume: 0.80,
        window_size: (1920 * 2, 1080 * 2),

        instance_mul,
        cameras,
    }
}
