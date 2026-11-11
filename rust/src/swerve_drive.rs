use godot::prelude::*;
use godot::classes::{RigidBody3D, IRigidBody3D, Input};

const SPEED_MULTIPLIER: f32 = 10.0;

#[derive(GodotClass)]
#[class(base=RigidBody3D)]
struct SwerveDrive {
    base: Base<RigidBody3D>,
}


#[godot_api]
impl IRigidBody3D for SwerveDrive {
    fn init(base: Base<RigidBody3D>) -> Self {
        Self {
            base,
        }
    }

    fn physics_process(&mut self, _delta: f64) {
        let base = &mut self.base_mut();
        let input = Input::singleton();

        let target_vx = input.get_axis("backward", "forward");
        let target_vy = input.get_axis("left", "right");
        let target_omega = input.get_axis("rotate_left", "rotate_right") / 10.0 + base.get_rotation().y;

        base.set_rotation(Vector3::new(0.0, target_omega, 0.0));
        base.apply_central_force(Vector3::new(target_vx * SPEED_MULTIPLIER, 0.0, target_vy * SPEED_MULTIPLIER));
    }
}
