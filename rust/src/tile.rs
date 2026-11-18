use godot::{
    classes::{CollisionShape3D, IRigidBody3D, RigidBody3D}, prelude::*,
};

use crate::random_colour_material::{RandomColourMaterial, get_random_colour};

#[derive(GodotClass)]
#[class(base=RigidBody3D, init)]
pub struct Tile {
    base: Base<RigidBody3D>,

    #[init(node = "MeshInstance3D")]
    mesh: OnReady<Gd<RandomColourMaterial>>,

    #[init(node = "CollisionShape3D")]
    collision: OnReady<Gd<CollisionShape3D>>,

    #[init(val = Color { r: 0.0, g: 0.0, b: 0.0, a: 0.0 })]
    pub colour: Color,

    #[init(val = Vector3 { x: 0.0, y: 0.0, z: 0.0 })]
    initial_position: Vector3,

    #[init(val = Vector3 { x: 0.0, y: 0.0, z: 0.0 })]
    initial_rotation: Vector3,

}

#[godot_api]
pub impl IRigidBody3D for Tile {
    fn ready(&mut self) {
        self.set_colour(get_random_colour());
        self.base_mut().set_freeze_enabled(true);
        let initial_position = self.base().get_position();
        let initial_rotation = self.base().get_rotation();
        self.initial_position = initial_position;
        self.initial_rotation = initial_rotation;
    }
}

#[godot_api]
pub impl Tile {
    pub fn set_colour(&mut self, colour: Color) {
        self.colour = colour;
        self.mesh.bind_mut().set_colour(colour);
    }

    pub fn set_activeness(&mut self, active: bool) {
        if active {
            let initial_position = self.initial_position;
            let initial_rotation = self.initial_rotation;
            let mut base = self.base_mut();
            base.set_position(initial_position);
            base.set_rotation(initial_rotation);
            base.set_freeze_enabled(true);
        } else {
            let mut base = self.base_mut();
            base.set_freeze_enabled(false);
            base.apply_impulse(Vector3 { x: 0.0, y: -0.3, z: 0.0 });
        }
    }
}
