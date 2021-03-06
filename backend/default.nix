{ sources ? import ../nix/sources.nix, pkgs ?
  import sources.nixpkgs { overlays = [ (import sources.nixpkgs-mozilla) ]; } }:
with pkgs;
let
  rust = import ../nix/rust.nix { inherit sources; };
  naersk = pkgs.callPackage sources.naersk {
    rustc = rust;
    cargo = rust;
  };
  gruvbox = pkgs.callPackage sources.gruvbox-css { };
  src = builtins.filterSource
    (path: type: type != "directory" || builtins.baseNameOf path != "target")
    ./.;
in naersk.buildPackage {
  name = "mi_backend";
  inherit src;
  buildInputs = with pkgs; [ openssl pkg-config sqlite libsodium ];
  GRUVBOX_CSS = "${gruvbox}/gruvbox.css";
  SODIUM_USE_PKG_CONFIG = "1";
  SODIUM_SHARED = "1";
}
