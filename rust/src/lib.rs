use godot::prelude::*;

mod swerve_drive;
mod camera_pivot;
mod arena;
mod tile;
mod random_colour_material;
mod main_scene;

struct GodotRustExtension;

#[gdextension]
unsafe impl ExtensionLibrary for GodotRustExtension {}

