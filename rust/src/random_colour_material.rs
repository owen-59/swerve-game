use godot::prelude::*;
use godot::classes::{IMeshInstance3D, MeshInstance3D};

#[derive(GodotClass)]
#[class(base=MeshInstance3D)]
struct RandomColourMaterial {
    base: Base<MeshInstance3D>,
}

#[godot_api]
impl IMeshInstance3D for RandomColourMaterial {
    fn init(base: Base<MeshInstance3D>) -> Self {
        Self {
            base,
        }
    }

    fn ready(&mut self) {
        let mut base = self.base_mut();

        let material = base.get_active_material(surface)
    }
}
