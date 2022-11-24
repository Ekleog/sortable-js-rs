let
  pkgs = import ./nix;
in
pkgs.stdenv.mkDerivation {
  name = "sortable-js-rs";
  buildInputs = (
    (with pkgs; [
      niv
      rust-analyzer-nightly
      trunk

      (fenix.combine (with fenix; [
        minimal.cargo
        minimal.rustc
        targets.wasm32-unknown-unknown.latest.rust-std
      ]))
    ])
  );
}
