{
  description = "HopeArtOrg/hope-re";

  inputs = {
    flake-parts.url = "github:hercules-ci/flake-parts";
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };

  outputs = inputs @ {flake-parts, ...}:
    flake-parts.lib.mkFlake {inherit inputs;} {
      systems = ["x86_64-linux" "aarch64-linux" "aarch64-darwin" "x86_64-darwin"];
      perSystem = {pkgs, ...}: {
        devShells.default =
          pkgs.mkShell {
            nativeBuildInputs = with pkgs; [
              pkg-config
              wrapGAppsHook4
              cargo
              nodejs
              rustc
              pnpm
              cargo-tauri
            ];

            buildInputs = with pkgs; [
              librsvg
              webkitgtk_4_1
            ];

            shellHook = "
         export XDG_DATA_DIRS=$GSETTINGS_SCHEMAS_PATH
        ";
          };
      };
    };
}
