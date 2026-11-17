use godot::{
    classes::{CollisionShape3D, IStaticBody3D, StaticBody3D},
    prelude::*,
};

use crate::random_colour_material::{RandomColourMaterial, get_random_colour};

#[derive(GodotClass)]
#[class(base=StaticBody3D, init)]
pub struct Tile {
    base: Base<StaticBody3D>,

    #[init(node = "MeshInstance3D")]
    mesh: OnReady<Gd<RandomColourMaterial>>,

    #[init(node = "CollisionShape3D")]
    collision: OnReady<Gd<CollisionShape3D>>,

    #[init(val = Color { r: 0.0, g: 0.0, b: 0.0, a: 0.0 })]
    pub colour: Color,
}

#[godot_api]
pub impl IStaticBody3D for Tile {
    fn ready(&mut self) {
        self.set_colour(get_random_colour());
    }
}

#[godot_api]
pub impl Tile {
    pub fn set_colour(&mut self, colour: Color) {
        self.colour = colour;
        self.mesh.bind_mut().set_colour(colour);
    }

    pub fn set_activeness(&mut self, active: bool) {
        self.base_mut().set_visible(active);
        self.collision.set_disabled(!active);
    }
}
