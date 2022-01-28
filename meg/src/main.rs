mod color;
mod instancer;
use crate::color::named_colorsets;
use kintaro::renderable::EventStreamConfig;
use kintaro::renderable::GlyphyConfig;
use kintaro::renderable::RenderableConfig;
use kintaro::renderable::ToyConfig;
use kintaro::{
    application::run, error::KintaroError, vertex::shape::Shape, Config, InstanceMul, RandIndex,
    RandPosition,
};

use crate::instancer::MegInstancer;

fn renderable_configs() -> Vec<RenderableConfig<'static>> {
    vec![
        // RenderableConfig::ImageRenderer(ImageRendererConfig {
        // image_path: "src/image_renderer/milo.png",
        // }),
        RenderableConfig::Toy(ToyConfig {
            shader_path: "src/toy.wgsl",
        }),
        RenderableConfig::EventStreams(EventStreamConfig {
            socool_path: "src/meg_gen.socool".to_string(),
            shader_path: "./src/shader.wgsl",
        }),
        RenderableConfig::Toy(ToyConfig {
            shader_path: "src/toy_glyphy.wgsl",
        }),
        RenderableConfig::Glyphy(GlyphyConfig::GlyphyNamedColorSetConfig {
            text: named_colorsets(),
            location: (0.05, 0.83),
            scale: 35.0,
        }),
        RenderableConfig::Glyphy(GlyphyConfig::GlypyTextConfig {
            text: vec![("open_neuro/ds003703/b2scmyvu_rest_01", "#ff2323")],
            location: (0.0, 0.0),
            scale: 50.0,
        }),
    ]
}

fn main() -> Result<(), KintaroError> {
    println!("Hello, Brain");
    let config = make_config();
    run("./src/meg_gen.socool", config)
}

pub fn make_config() -> Config<'static> {
    let instance_mul = InstanceMul {
        x: 9.0,
        y: 19.0,
        z: 1.0,
        life: 2.0,
        size: 23.0,
        length: 1.0,
    };
    let (cameras, instance_mul) = Config::handle_save(instance_mul);
    // pub composition_name: &'a str,
    // pub renderable_configs: Vec<RenderableConfig<'a>>,
    // pub volume: f32,
    // pub window_size: (u32, u32),
    // pub cameras: Vec<CameraConfig>,
    // pub accumulation: bool,
    // pub shape: Shape,
    // pub instance_mul: InstanceMul,
    // pub instancer: Box<dyn Instancer>,
    Config {
        composition_name: "Meg",
        renderable_configs: renderable_configs(),
        instancer: Box::new(MegInstancer {}),
        instance_mul,
        accumulation: false,
        volume: 0.20,
        window_size: (1920 * 2, 1080 * 2),
        cameras,
        shape: Shape {
            n_vertices: 30,
            n_indices: 30,
            position: Box::new(RandPosition),
            color: Box::new(color::color_map()),
            indices: Box::new(RandIndex),
        },
    }
}
