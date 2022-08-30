{
pkgs ? import (
  fetchTarball {
    url = "https://github.com/NixOS/nixpkgs/archive/1cc8a7ba8844f68a646da509a3976b52f406a28c.tar.gz";
    sha256 = "sha256:1dxsv600lcdaj8dl87i0nxj7k057ddpi299ba8zvqc7knh8b7642";
  }
) {}}:

pkgs.mkShell {
  buildInputs = [
    pkgs.bash
    pkgs.rustup
    pkgs.wasmtime
    pkgs.binaryen
    (pkgs.php81.buildEnv {
        extensions = { all, ... }: with all; [ ffi ];
        extraConfig = "ffi.enable=true";
    })
  ];
}
