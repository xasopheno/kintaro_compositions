mod color;
mod color2;
mod instancer;
mod instancer2;
use kintaro::config::{named_colorsets, FramePass};
use kintaro::error::KintaroError;
use kintaro::renderable::EventStreamConfig;
use kintaro::renderable::GlyphyConfig;
use kintaro::renderable::OrigamiConfig;
use kintaro::renderable::RenderableConfig;
use kintaro::renderable::ToyConfig;
use kintaro::{
    application::run,
    vertex::shape::{RandCircumference, Shape},
    Config, InstanceMul, RandIndex, RandPosition,
};

fn main() -> Result<(), KintaroError> {
    println!("Hello, Origamis");
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
            RenderableConfig::Origami(OrigamiConfig {
                shader_path: "src/origami_shader.wgsl",
                n_indices: 3000,
                n_vertices: 8,
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
        socool_path: "./src/template3.socool",
        frame_passes: frame_passes(),
        composition_name: "Daemon",

        volume: 0.80,
        window_size: (1920 * 2, 1080 * 2),

        instance_mul,
        cameras,
    }
}
