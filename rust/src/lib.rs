use godot::prelude::*;

mod swerve_drive;
mod camera_pivot;

struct GodotRustExtension;

#[gdextension]
unsafe impl ExtensionLibrary for GodotRustExtension {}

