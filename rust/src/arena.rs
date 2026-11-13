use godot::prelude::*;
use godot::classes::{INode3D, Node3D};

const TILE_SCENE_PATH: &str = "res://scenes/tile.tscn";
const TILE_OFFSET: f32 = 2.0;

#[derive(GodotClass)]
#[class(tool, base=Node3D)]
struct Arena {
    base: Base<Node3D>,
}

#[godot_api]
impl INode3D for Arena {
    fn init(base: Base<Node3D>) -> Self {
        Self {
            base,
        }
    }
    
    fn ready(&mut self) {
        let tile_scene: Gd<PackedScene> = load(TILE_SCENE_PATH);
        let owner = self.to_gd().upcast::<Node3D>();
        let mut base = self.base_mut();

        for i in -2..=2 {
            for j in -2..=2 {
                let mut tile: Gd<Node3D> = tile_scene.instantiate().unwrap().cast();

                tile.set_position(Vector3 { x: i as f32 * TILE_OFFSET, y: 0.0, z: j as f32 * TILE_OFFSET });
                base.add_child(&tile.clone());
                tile.set_owner(&owner);

            }
        }
    }
}
