use godot::prelude::*;
use godot::classes::{INode3D, Node3D};

#[derive(GodotClass)]
#[class(base=Node3D)]
struct CameraPivot {
    base: Base<Node3D>,
}

#[godot_api]
impl INode3D for CameraPivot {
    fn init(base: Base<Node3D>) -> Self {
        Self {
            base,
        }
    }

    fn physics_process(&mut self, _delta: f64) {
        let mut base = self.base_mut();
        let parent = base.get_parent();
        if let Some(parent) = parent {
            base.set_global_position(parent.cast::<Node3D>().get_global_position());
        } 
    }
}
