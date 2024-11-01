{ }:
let pkgs = import <nixpkgs> { };
in pkgs.mkShell {
  name = "avc";

  buildInputs = with pkgs; [ cargo rustc rust-analyzer rustfmt clippy ];
}

