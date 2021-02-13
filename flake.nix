{
  description = "";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-20.09";
    utils.url = "github:numtide/flake-utils";
    # for rustChannelOF
    mozilla = { url = "github:mozilla/nixpkgs-mozilla"; flake = false; };
    naersk.url = "github:nmattia/naersk";
  };

  outputs =
    { self
    , nixpkgs
    , utils
    , mozilla
    , naersk
    , ... } @ inputs:
    utils.lib.eachDefaultSystem (system:
      let

        rustOverlay = final: prev:
          let
            rustChannel = prev.rustChannelOf {
              channel = "1.49.0";
              sha256 = "sha256-KCh2UBGtdlBJ/4UOqZlxUtcyefv7MH1neoVNV4z0nWs=";
            };
          in
            {
              inherit rustChannel;
              rustc = rustChannel.rust;
              cargo = rustChannel.rust;
            };

        naersk-lib = naersk.lib."${system}";

        pkgs = import nixpkgs {
          inherit system;
          config = { };
          overlays = [
            (import "${mozilla}/rust-overlay.nix")
            rustOverlay
            naersk.overlay
          ];
        };
      in
      {
        devShell =  pkgs.mkShell {
          name = "open-here";

          buildInputs = with pkgs; [
            # For carog, clippy, rustfmt, etc.
            # rust-src is needed by rust-analyzer
            (rustChannel.rust.override { extensions = [ "rust-src" ]; })

            cargo-edit
          ];
        };

        packages =
          let
            open-here = naersk-lib.buildPackage ./.; #{ src = ./.; cargoOptions = [];};
          in
          {
            inherit open-here;
          };

        defaultPackage = self.packages.${system}.open-here;

      }
    );
}
