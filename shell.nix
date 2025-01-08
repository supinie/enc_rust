with import <nixpkgs> {};
mkShell {
    buildInputs = [ rustup gcc bacon ];
}
