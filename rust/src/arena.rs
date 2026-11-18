use godot::classes::{INode3D, Node3D, Time, Timer};
use godot::prelude::*;
use rand::seq::IteratorRandom;

use crate::random_colour_material::get_random_colour;
use crate::tile::Tile;

const TILE_SCENE_PATH: &str = "res://scenes/tile.tscn";
const TILE_OFFSET: f32 = 2.0;

#[derive(GodotClass)]
#[class(base=Node3D, init)]
pub struct Arena {
    base: Base<Node3D>,

    #[init(val = Array::new())]
    tiles: Array<Gd<Tile>>,
    
    #[init(val = Time::singleton().get_ticks_msec())]
    ms_at_last_reset: u64,
    
    #[init(val = true)]
    is_safe: bool,
    
    #[init(val = Color { r: 1.0, g: 1.0, b: 1.0, a: 1.0 })]
    pub current_colour: Color,

    #[init(val = 1)]
    level: u32,

    #[init(node = "SafeTimer")]
    safe_timer: OnReady<Gd<Timer>>,

    #[init(node = "UnsafeTimer")]
    unsafe_timer: OnReady<Gd<Timer>>
}

#[godot_api]
impl INode3D for Arena {
    fn ready(&mut self) {
        let tile_scene: Gd<PackedScene> = load(TILE_SCENE_PATH);
        let owner = self.to_gd().upcast::<Node3D>();

        for i in -4..=4 {
            for j in -4..=4 {
                let mut tile: Gd<Tile> = tile_scene.instantiate().unwrap().cast::<Tile>();
                let mut base = self.base_mut();

                tile.set_position(Vector3 {
                    x: i as f32 * TILE_OFFSET,
                    y: 0.0,
                    z: j as f32 * TILE_OFFSET,
                });
                base.add_child(&tile);
                tile.set_owner(&owner);

                drop(base);

                self.tiles.push(&tile);
            }
        }

        let safe_timer = (*self.safe_timer).clone();
        let unsafe_timer = (*self.unsafe_timer).clone();
        
        safe_timer
            .signals()
            .timeout()
            .connect_other(&self.to_gd(), Arena::on_safe_end);

        unsafe_timer
            .signals()
            .timeout()
            .connect_other(&self.to_gd(), Arena::on_unsafe_end);
    }
}

#[godot_api]
pub impl Arena {
    #[signal]
    pub fn colour_changed(colour: Color);

    #[signal]
    pub fn level_changed(level: u32);

    #[signal]
    pub fn tiles_fell();

    #[func]
    pub fn set_colour(&mut self, colour: Color) {
        self.current_colour = colour;
        self.run_deferred(move |this| {
            this.signals().colour_changed().emit(colour);
        });
    }

    #[func]
    pub fn set_level(&mut self, level: u32) {
        self.level = level;
        self.run_deferred(move |this| {
            this.signals().level_changed().emit(level);
        });
    }
}

impl Arena {
    pub fn on_unsafe_end(&mut self) {
        let colour = get_random_colour();
        let current_time = Time::singleton().get_ticks_msec();
        self.is_safe = true;
        self.set_colour(colour);
        self.ms_at_last_reset = current_time;
        self.signals().colour_changed().emit(colour);
        for mut tile in self.tiles.iter_shared() {
            let mut tile_bind = tile.bind_mut();
            tile_bind.set_colour(get_random_colour());
            tile_bind.set_activeness(true);
        }

        let mut rng = rand::rng();
        if let Some(mut random_tile) = self.tiles.iter_shared().choose(&mut rng) {
            let mut tile_bind = random_tile.bind_mut();
            tile_bind.set_colour(self.current_colour);
        }
        let new_level = self.level + 1;
        self.set_level(new_level);
        self.safe_timer.start();
    }

    pub fn on_safe_end(&mut self) {
        self.is_safe = false;
        self.signals().tiles_fell().emit();
        for mut tile in self.tiles.iter_shared() {
            let is_active = tile.bind().colour == self.current_colour;
            tile.bind_mut().set_activeness(is_active);
        }
        self.unsafe_timer.start();
    }
}
