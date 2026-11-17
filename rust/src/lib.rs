use godot::prelude::*;

mod arena;
mod camera_pivot;
mod main_scene;
mod random_colour_material;
mod swerve_drive;
mod tile;
mod hud;

struct GodotRustExtension;

#[gdextension]
unsafe impl ExtensionLibrary for GodotRustExtension {}
