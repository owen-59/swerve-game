use godot::prelude::*;
use godot::classes::{INode3D, Node3D};
use crate::arena::Arena;
use crate::swerve_drive::SwerveDrive;

#[derive(GodotClass)]
#[class(base=Node3D, init)]
struct MainScene {
    base: Base<Node3D>,

    #[init(node = "Arena")]
    arena: OnReady<Gd<Arena>>,

    #[init(node = "SwerveDrive")]
    player: OnReady<Gd<SwerveDrive>>
}

#[godot_api]
impl INode3D for MainScene {
    fn process(&mut self, _delta: f64) {
        let arena = (*self.arena).clone();
        let player = (*self.player).clone();
        arena.signals().colour_changed().connect_other(&player, SwerveDrive::set_colour);
    }
}
