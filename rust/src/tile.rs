use godot::prelude::*;
use godot::classes::{IRigidBody3D, RigidBody3D};

#[derive(GodotClass)]
#[class(base=RigidBody3D)]
struct Tile {
    base: Base<RigidBody3D>,
}

#[godot_api]
impl IRigidBody3D for Tile {
    fn init(base: Base<RigidBody3D>) -> Self {
        Self {
            base,
        }
    }

    fn ready(&mut self) {}
}
