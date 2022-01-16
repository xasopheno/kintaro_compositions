mod color;
mod instancer;
use kintaro::{
    application::run, vertex::shape::Shape, Config, Error, InstanceMul, RandIndex, RandPosition,
};

use crate::instancer::MegInstancer;

fn main() -> Result<(), Error> {
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
    Config {
        instance_shader: "./src/shader.wgsl".into(),
        toy_shader: "./src/toy.wgsl".into(),
        instancer: Box::new(MegInstancer {}),
        instance_mul,
        accumulation: false,
        filename: "kintaro".into(),
        volume: 0.20,
        window_size: (1920 * 2, 1080 * 2),
        cameras,
        text: Some(color::named_colorsets()),
        shape: Shape {
            n_vertices: 30,
            n_indices: 30,
            position: Box::new(RandPosition),
            color: Box::new(color::color_map()),
            indices: Box::new(RandIndex),
        },
    }
}
