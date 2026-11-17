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
    let colour_id = random_range(1..=6);
    match colour_id {
        1 => Color {
            r: 1.0,
            g: 0.0,
            b: 0.0,
            a: 1.0,
        },
        2 => Color {
            r: 0.0,
            g: 1.0,
            b: 0.0,
            a: 1.0,
        },
        3 => Color {
            r: 0.0,
            g: 0.0,
            b: 1.0,
            a: 1.0,
        },
        4 => Color {
            r: 0.0,
            g: 1.0,
            b: 1.0,
            a: 1.0,
        },
        5 => Color {
            r: 1.0,
            g: 0.0,
            b: 1.0,
            a: 1.0,
        },
        _ => Color {
            r: 1.0,
            g: 1.0,
            b: 0.0,
            a: 1.0,
        },
    }
}
