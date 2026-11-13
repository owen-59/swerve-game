use godot::prelude::*;

mod swerve_drive;
mod camera_pivot;
mod arena;
mod tile;
mod random_colour_material;

struct GodotRustExtension;

#[gdextension]
unsafe impl ExtensionLibrary for GodotRustExtension {}

