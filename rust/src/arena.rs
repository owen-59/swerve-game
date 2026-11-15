use godot::prelude::*;
use godot::classes::{INode3D, Node3D, Time};

use crate::random_colour_material::get_random_colour;
use crate::tile::Tile;

const TILE_SCENE_PATH: &str = "res://scenes/tile.tscn";
const TILE_OFFSET: f32 = 2.0;

#[derive(GodotClass)]
#[class(tool, base=Node3D)]
struct Arena {
    base: Base<Node3D>,
    tiles: Array<Gd<Tile>>,
    ms_at_last_reset: u64,
}

#[godot_api]
impl INode3D for Arena {
    fn init(base: Base<Node3D>) -> Self {
        Self {
            base,
            tiles: Array::new(),
            ms_at_last_reset: Time::singleton().get_ticks_msec()
        }
    }
    
    fn ready(&mut self) {
        let tile_scene: Gd<PackedScene> = load(TILE_SCENE_PATH);
        let owner = self.to_gd().upcast::<Node3D>();

        for i in -2..=2 {
            for j in -2..=2 {
                let mut tile: Gd<Tile> = tile_scene.instantiate().unwrap().cast::<Tile>();
                let mut base = self.base_mut();

                tile.set_position(Vector3 { x: i as f32 * TILE_OFFSET, y: 0.0, z: j as f32 * TILE_OFFSET });
                base.add_child(&tile);
                tile.set_owner(&owner);

                drop(base);

                self.tiles.push(&tile);
            }
        }
    }

    fn process(&mut self, _delta: f64) {
        let time = Time::singleton();
        let current_time = time.get_ticks_msec();

        if current_time > self.ms_at_last_reset + 1000 {
            self.ms_at_last_reset = current_time;
            for mut tile in self.tiles.iter_shared() {
                tile.bind_mut().set_colour(get_random_colour());
            }
        }
    }
}
