{
  description = "A Slint Rust development environment for rustbar";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, utils }:
    utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
        
        runtimeLibs = with pkgs; [
          libGL
          libxkbcommon
          wayland
          fontconfig
        ];
      in
      {
        devShells.default = pkgs.mkShell {
          nativeBuildInputs = with pkgs; [
            pkg-config
            cmake
            wayland-protocols
            rustc
            cargo
            cargo-watch
            rust-analyzer
          ];

          buildInputs = runtimeLibs;

          shellHook = ''
            export LD_LIBRARY_PATH=${pkgs.lib.makeLibraryPath runtimeLibs}:$LD_LIBRARY_PATH
            export WINIT_UNIX_BACKEND=wayland
            export QT_QPA_PLATFORM=wayland
            export GDK_BACKEND=wayland
          '';
        };
      });
}
