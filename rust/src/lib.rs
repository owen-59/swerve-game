use godot::prelude::*;

mod swerve_drive;
mod optimise;

struct GodotRustExtension;

#[gdextension]
unsafe impl ExtensionLibrary for GodotRustExtension {}

