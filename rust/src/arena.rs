use godot::prelude::*;
use godot::classes::{INode3D, Node3D, Time};

use crate::random_colour_material::get_random_colour;
use crate::tile::Tile;

const TILE_SCENE_PATH: &str = "res://scenes/tile.tscn";
const TILE_OFFSET: f32 = 2.0;
const COLOUR_CHANGE_MS: u64 = 5000;
const SAFE_MS: u64 = 3000;

#[derive(GodotClass)]
#[class(base=Node3D)]
pub struct Arena {
    base: Base<Node3D>,
    tiles: Array<Gd<Tile>>,
    ms_at_last_reset: u64,
    is_safe: bool,
    pub current_colour: Color
}

#[godot_api]
impl INode3D for Arena {
    fn init(base: Base<Node3D>) -> Self {
        Self {
            base,
            tiles: Array::new(),
            ms_at_last_reset: Time::singleton().get_ticks_msec(),
            is_safe: true,
            current_colour: Color { r: 1.0, g: 1.0, b: 1.0, a: 1.0 }    
        }
    }
    
    fn ready(&mut self) {
        let tile_scene: Gd<PackedScene> = load(TILE_SCENE_PATH);
        let owner = self.to_gd().upcast::<Node3D>();

        for i in -4..=4 {
            for j in -4..=4 {
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

        if current_time > self.ms_at_last_reset + COLOUR_CHANGE_MS {
            let colour = get_random_colour();
            self.is_safe = true;
            self.set_colour(colour);
            self.ms_at_last_reset = current_time;
            self.signals().colour_changed().emit(colour);
            for mut tile in self.tiles.iter_shared() {
                let mut tile_bind = tile.bind_mut();
                tile_bind.set_colour(get_random_colour());
                tile_bind.set_activeness(true);
            }
        }

        if current_time > self.ms_at_last_reset + SAFE_MS && self.is_safe {
            for mut tile in self.tiles.iter_shared() {
                let is_active = tile.bind().colour == self.current_colour;
                tile.bind_mut().set_activeness(is_active);
            }
        }
    }
}

#[godot_api]
pub impl Arena {
    #[signal]
    pub fn colour_changed(colour: Color);

    #[func]
    pub fn set_colour(&mut self, colour: Color) {
        self.current_colour = colour;
        self.run_deferred(move |this| {
            this.signals().colour_changed().emit(colour);
        });
    }
}
