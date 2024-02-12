{
  description = "critique.rs";

  inputs = {
    nixpkgs.url = "nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = {
    self,
    nixpkgs,
    flake-utils,
    ...
  } @ inputs: let
  in
    flake-utils.lib.eachDefaultSystem (
      system: let
        pkgs = import nixpkgs {inherit system;};
        buildInputs = with pkgs; [
          cargo
          rustc
          openssl
          pkg-config
        ];
      in {
        devShells.default = pkgs.mkShell {
          buildInputs = buildInputs;
        };
      }
    );
}
