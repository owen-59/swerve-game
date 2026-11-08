{ pkgs, options, lib, config, inputs, ... }:

{
    languages.rust = {
        enable = true;
        channel = "nightly";
        targets = [ "wasm32-unknown-emscripten" ];
        components = options.languages.rust.components.default ++ [ "rust-src" ];
    };
    packages = [ 
        pkgs.godot 
        pkgs.emscripten
    ];
}
