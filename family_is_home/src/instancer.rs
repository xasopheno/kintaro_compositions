use cgmath::Rotation3;
use kintaro::{Instance, Instancer, InstancerInput, InstancerOutput};
use rand::Rng;

#[derive(Debug, Clone)]
pub struct MegInstancer {}

impl Instancer for MegInstancer {
    fn update_instance(&self, instance: &mut Instance, dt: f32) {
        let mut rng = rand::thread_rng();
        instance.life -= dt * 0.1;
        instance.position.x += 30.0 * (2.0 - instance.life) * f32::signum(instance.position.x);
        instance.position.x += rng.gen_range(-10.0..10.0);
        instance.position.y += 30.0 * (2.0 - instance.life) * f32::signum(instance.position.y);
        instance.position.y +=
            f32::sin(dt * 1000.0 * f32::sin(instance.position.z) * f32::tan(instance.life));
        instance.position.z -= 30.0 * (2.0 - instance.life);
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
            size: input.size,
            rotation,
        }
    }
}
