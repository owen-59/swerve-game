use crate::arena::Arena;
use crate::random_colour_material::get_random_colour;
use crate::swerve_drive::SwerveDrive;
use godot::classes::{INode3D, Node3D};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Node3D, init)]
struct MainScene {
    base: Base<Node3D>,

    #[init(node = "Arena")]
    arena: OnReady<Gd<Arena>>,

    #[init(node = "SwerveDrive")]
    player: OnReady<Gd<SwerveDrive>>,
}

#[godot_api]
impl INode3D for MainScene {
    fn ready(&mut self) {
        self.run_deferred(|this| {
            let colour = get_random_colour();
            this.arena.bind_mut().set_colour(colour);
            this.player.bind_mut().set_colour(colour);
        });
    }

    fn process(&mut self, _delta: f64) {
        let arena = (*self.arena).clone();
        let player = (*self.player).clone();
        arena
            .signals()
            .colour_changed()
            .connect_other(&player, SwerveDrive::set_colour);
    }
}
