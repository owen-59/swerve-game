use godot::classes::{IMeshInstance3D, MeshInstance3D, StandardMaterial3D};
use godot::prelude::*;
use rand::random_range;

#[derive(GodotClass)]
#[class(base=MeshInstance3D)]
pub struct RandomColourMaterial {
    base: Base<MeshInstance3D>,
    // #[init(node = "MeshInstance3D")]
    // mesh: OnReady<Gd<>>
}

#[godot_api]
impl IMeshInstance3D for RandomColourMaterial {
    fn init(base: Base<MeshInstance3D>) -> Self {
        Self { base }
    }
}

#[godot_api]
impl RandomColourMaterial {
    #[func]
    pub fn set_colour(&mut self, colour: Color) {
        let mut base = self.base_mut();
        let mut material = StandardMaterial3D::new_gd();
        material.set_albedo(colour);

        base.set_surface_override_material(0, &material);
    }
}

pub fn get_random_colour() -> Color {
    Color {
        r: random_range(0..=2) as f32 / 2.0,
        g: random_range(0..=2) as f32 / 2.0,
        b: random_range(0..=2) as f32 / 2.0,
        a: 1.0
    }
}
