use godot::classes::{IRigidBody3D, Input, RigidBody3D};
use godot::prelude::*;

use crate::random_colour_material::RandomColourMaterial;

const SPEED_MULTIPLIER: f32 = 10.0;

#[derive(GodotClass)]
#[class(base=RigidBody3D, init)]
pub struct SwerveDrive {
    base: Base<RigidBody3D>,

    #[init(node = "MeshInstance3D")]
    mesh: OnReady<Gd<RandomColourMaterial>>,
}

#[godot_api]
impl IRigidBody3D for SwerveDrive {
    fn ready(&mut self) {
        self.set_colour(Color {
            r: 1.0,
            g: 1.0,
            b: 1.0,
            a: 1.0,
        });
    }

    fn physics_process(&mut self, _delta: f64) {
        let base = &mut self.base_mut();
        let input = Input::singleton();

        let target_vx = input.get_axis("backward", "forward");
        let target_vy = input.get_axis("left", "right");
        let target_omega = -input.get_axis("rotate_left", "rotate_right");

        base.apply_torque(Vector3::new(0.0, target_omega, 0.0));
        base.apply_central_force(Vector3::new(
            target_vx * SPEED_MULTIPLIER,
            0.0,
            target_vy * SPEED_MULTIPLIER,
        ));
    }
}

#[godot_api]
pub impl SwerveDrive {
    pub fn set_colour(&mut self, colour: Color) {
        self.mesh.bind_mut().set_colour(colour);
    }
}
