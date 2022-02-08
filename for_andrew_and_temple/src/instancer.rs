use cgmath::Rotation3;
use kintaro::{Instance, Instancer, InstancerInput, InstancerOutput};
use rand::Rng;

#[derive(Debug, Clone)]
pub struct MegInstancer {}

impl Instancer for MegInstancer {
    fn update_instance(&self, instance: &mut Instance, dt: f32) {
        instance.life -= dt * 0.1;
        instance.position.x += 800.0 * (2.0 - instance.life) * f32::signum(instance.position.x);
        // instance.position.y += 100.0 * f32::cos(300000.0 * (2.0 - instance.life));
        // instance.position.y += f32::sin(3.0 * (2.0 - instance.life));
        // instance.position.y += 700.0 * (2.0 - instance.life) * f32::signum(instance.position.y);
        // instance.position.y += f32::sin(
        // dt * 0.1
        // * f32::sin(instance.position.x / instance.position.y)
        // * f32::tan(instance.life),
        // );
    }

    fn op4d_to_instance_transformation(&self, input: InstancerInput) -> InstancerOutput {
        let mut rng = rand::thread_rng();
        let rotation = cgmath::Quaternion::from_axis_angle(
            cgmath::Vector3::unit_x(),
            cgmath::Deg(rng.gen_range(-10.8..10.8)),
        );

        InstancerOutput {
            x: input.x,
            y: input.y,
            z: input.z,
            length: input.length,
            life: input.life,
            size: input.size,
            rotation,
        }
    }
}
