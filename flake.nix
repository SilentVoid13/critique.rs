{
  description = "critique.rs";

  inputs = {
    nixpkgs.url = "nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    {
      nixpkgs,
      flake-utils,
      fenix,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ fenix.overlays.default ];
        };

        toolchain =
          with fenix.packages.${system};
          combine [
            # default / complete / latest / minimal = nightly
            # stable
            # beta

            default.rustc
            default.cargo
            default.clippy
            default.rustfmt
            pkgs.rust-analyzer
            targets.wasm32-unknown-unknown.latest.rust-std
          ];

        buildInputs = with pkgs; [
          toolchain
          openssl
          pkg-config
          llvmPackages.bintools-unwrapped
          tailwindcss_4
        ];
        nativeBuildInputs = with pkgs; [
        ];
      in
      {
        devShell = pkgs.mkShell {
          inherit buildInputs nativeBuildInputs;
          LD_LIBRARY_PATH = "${pkgs.lib.makeLibraryPath buildInputs}";
        };
      }
    );
}
