use cgmath::Rotation3;
use kintaro::{Instance, Instancer, InstancerInput, InstancerOutput};
use rand::Rng;

#[derive(Debug, Clone)]
pub struct Instancer1 {}

impl Instancer for Instancer1 {
    fn update_instance(&self, instance: &mut Instance, dt: f32) {
        let mut rng = rand::thread_rng();
        instance.life -= dt * 0.1;
        instance.position.x += f32::sin(instance.life * 7.0) * 3500.0 * (2.0 - instance.life);
        // * f32::signum(instance.position.x);
        // instance.position.x += rng.gen_range(-10.0..10.0);
        instance.position.y -= f32::sin(instance.life * 5.0) * -2000.0 * (2.0 - instance.life);
        // * f32::signum(instance.position.y);
        instance.position.y +=
            f32::cos(dt * 1500.0 * f32::sin(instance.position.z) * f32::tan(instance.life));
        instance.position.z -= 1.0 * (2.0 - instance.life);
    }

    fn op4d_to_instance_transformation(&self, input: InstancerInput) -> InstancerOutput {
        let mut rng = rand::thread_rng();
        let rotation = cgmath::Quaternion::from_axis_angle(
            cgmath::Vector3::unit_x(),
            cgmath::Deg(rng.gen_range(-10.9..10.9)),
        );

        InstancerOutput {
            x: -input.x,
            y: input.y,
            z: input.z,
            length: input.length,
            life: input.life,
            size: input.size * 5.0,
            rotation,
        }
    }
}
