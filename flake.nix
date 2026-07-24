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
        devShells =
          {
            default = let
              libraries = with pkgs; [
                librsvg
                webkitgtk_4_1
                dbus.dev
                glib.dev
                gtk3
                libsoup_3
              ];
            in
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

                buildInputs = libraries;

                shellHook = ''
                  export XDG_DATA_DIRS="$GSETTINGS_SCHEMAS_PATH"

                  export LD_LIBRARY_PATH="${pkgs.lib.makeLibraryPath libraries}:$LD_LIBRARY_PATH"
                '';
              };
          }
          // pkgs.lib.optionalAttrs (pkgs ? buildFHSEnv || pkgs ? buildFHSUserEnv) {
            fhs = let
              builder = pkgs.buildFHSEnv or pkgs.buildFHSUserEnv;

              fhsEnv = builder {
                name = "tauri-fhs-env";
                targetPkgs = pkgs:
                  with pkgs; [
                    pkg-config
                    wrapGAppsHook4
                    cargo
                    nodejs
                    rustc
                    pnpm
                    cargo-tauri
                    librsvg
                    webkitgtk_4_1.dev
                    dbus.dev
                    glib.dev
                    gtk3.dev
                    libsoup_3.dev
                    gdk-pixbuf
                    gdk-pixbuf.dev
                    pango.dev
                    cairo.dev
                    atk.dev
                    harfbuzz.dev
                    openssl.dev
                    zlib
                    zlib.dev
                    freetype.dev
                    fontconfig.dev
                    libx11.dev
                    glib-networking
                    gst_all_1.gstreamer
                    gst_all_1.gst-plugins-base
                    gst_all_1.gst-plugins-good
                    gst_all_1.gst-plugins-bad
                    mesa
                    egl-wayland
                    hicolor-icon-theme
                    gsettings-desktop-schemas
                    rustfmt
                    clippy
                    fish
                  ];
                profile = ''
                  export XDG_DATA_DIRS="${pkgs.shared-mime-info}/share:${pkgs.gtk3}/share/gsettings-schemas/${pkgs.gtk3.name}:${pkgs.gsettings-desktop-schemas}/share/gsettings-schemas/${pkgs.gsettings-desktop-schemas.name}:/usr/share:''${XDG_DATA_DIRS:-}"
                  export LD_LIBRARY_PATH="/usr/lib:/lib:$LD_LIBRARY_PATH"

                  export PKG_CONFIG_PATH="/usr/lib/pkgconfig:/usr/share/pkgconfig:/usr/lib/x86_64-linux-gnu/pkgconfig:''${PKG_CONFIG_PATH:-}"

                  export GDK_BACKEND=wayland
                  export GALLIUM_DRIVER=llvmpipe
                  export MESA_LOADER_DRIVER_OVERRIDE=llvmpipe

                  export WEBKIT_DISABLE_SANDBOX_THIS_IS_DANGEROUS=1
                  export WEBKIT_DISABLE_DMABUF_RENDERER=1
                  export WEBKIT_DISABLE_COMPOSITING_MODE=1

                  export LIBGL_ALWAYS_SOFTWARE=1
                '';
                runScript = "fish";
              };
            in
              pkgs.mkShell {
                packages = [fhsEnv];
                shellHook = ''
                  exec ${fhsEnv}/bin/tauri-fhs-env
                '';
              };
          };
      };
    };
}
